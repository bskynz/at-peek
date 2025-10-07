// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::components::bulk_analysis::PostWithLabels;
use atproto_client::{
    create_session, resolve_did, resolve_handle, AtRecord, Did, Handle, LabelCollection,
    LabelerClient, PostClient,
};
use std::collections::HashMap;

// Re-export these types from utils since they're used in the public API
pub use crate::components::bulk_analysis::{BulkAnalysisStats, UserInfo};

/// Authenticate with Bluesky
pub async fn authenticate(handle: &str, password: &str) -> Result<String, String> {
    create_session(handle, password)
        .await
        .map(|session| session.access_jwt)
        .map_err(|e| format!("Authentication failed: {}", e))
}

/// Fetch labels for a given subject (handle, DID, or AT-URI) from multiple sources
pub async fn fetch_labels(
    input: &str,
    auth_token: Option<String>,
) -> Result<LabelCollection, String> {
    let bsky_labeler = if let Some(token) = &auth_token {
        LabelerClient::new_authenticated(token.clone())
    } else {
        LabelerClient::new()
    };

    // Strip @ symbol if present (users might type @alice.bsky.social)
    let input = input.trim_start_matches('@');

    // Determine what type of input we have
    let (subject, did_opt) = if input.starts_with("at://") {
        // AT-URI
        (input.to_string(), None)
    } else if input.starts_with("did:") {
        // DID
        let did = atproto_client::Did::new(input.to_string());
        (did.as_str().to_string(), Some(did))
    } else if input.contains('.') {
        // Assume it's a handle - resolve to DID first
        let handle = Handle::new(input.to_string());
        let did = resolve_handle(&handle)
            .await
            .map_err(|e| format!("Failed to resolve handle: {}", e))?;
        (did.as_str().to_string(), Some(did))
    } else {
        return Err("Invalid input format. Expected handle, DID, or AT-URI".to_string());
    };

    // Query Bluesky's moderation service
    let mut all_labels = match bsky_labeler
        .query_labels(std::slice::from_ref(&subject))
        .await
    {
        Ok(collection) => collection.labels,
        Err(e) => {
            // Propagate authentication errors to the user
            if matches!(e, atproto_client::Error::AuthenticationRequired(_)) {
                return Err(e.to_string());
            }
            log::warn!("Failed to query Bluesky labeler: {}", e);
            Vec::new()
        }
    };

    // If we have a DID, also query the user's PDS for admin labels
    if let Some(did) = did_opt {
        if let Ok(pds_endpoint) = resolve_did(&did).await {
            let pds_labeler = if let Some(token) = &auth_token {
                LabelerClient::with_url(pds_endpoint).with_auth(token.clone())
            } else {
                LabelerClient::with_url(pds_endpoint)
            };
            match pds_labeler.query_labels(&[subject]).await {
                Ok(collection) => {
                    all_labels.extend(collection.labels);
                }
                Err(e) => {
                    // Propagate authentication errors to the user
                    if matches!(e, atproto_client::Error::AuthenticationRequired(_)) {
                        return Err(e.to_string());
                    }
                    log::warn!("Failed to query PDS: {}", e);
                }
            }
        }
    }

    Ok(LabelCollection {
        labels: all_labels,
        labeler_did: "multiple".to_string(),
        query_timestamp: chrono::Utc::now(),
    })
}

/// Shorten a DID for display
pub fn shorten_did(did: &str) -> String {
    if did.len() > 20 {
        format!("{}...{}", &did[..12], &did[did.len() - 5..])
    } else {
        did.to_string()
    }
}

/// Format ISO 8601 timestamp as human-readable
pub fn format_timestamp(iso8601: &str) -> String {
    chrono::DateTime::parse_from_rfc3339(iso8601)
        .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
        .unwrap_or_else(|_| iso8601.to_string())
}

/// Calculate duration between two timestamps and format as human-readable string
pub fn calculate_duration(from_timestamp: &str, to_timestamp: &str) -> String {
    let from = chrono::DateTime::parse_from_rfc3339(from_timestamp);
    let to = chrono::DateTime::parse_from_rfc3339(to_timestamp);

    match (from, to) {
        (Ok(from_dt), Ok(to_dt)) => {
            let duration = to_dt.signed_duration_since(from_dt);

            let days = duration.num_days();
            let hours = duration.num_hours() % 24;
            let minutes = duration.num_minutes() % 60;
            let seconds = duration.num_seconds() % 60;

            if days > 0 {
                if hours > 0 {
                    format!(
                        "{} day{}, {} hour{}",
                        days,
                        if days == 1 { "" } else { "s" },
                        hours,
                        if hours == 1 { "" } else { "s" }
                    )
                } else {
                    format!("{} day{}", days, if days == 1 { "" } else { "s" })
                }
            } else if hours > 0 {
                if minutes > 0 {
                    format!(
                        "{} hour{}, {} min",
                        hours,
                        if hours == 1 { "" } else { "s" },
                        minutes
                    )
                } else {
                    format!("{} hour{}", hours, if hours == 1 { "" } else { "s" })
                }
            } else if minutes > 0 {
                format!("{} min, {} sec", minutes, seconds)
            } else {
                format!("{} sec", seconds)
            }
        }
        _ => "Unknown".to_string(),
    }
}

/// Fetch likes for a post from the AppView
async fn fetch_likes(post_uri: &str) -> Result<(usize, Vec<UserInfo>), String> {
    let client = reqwest::Client::new();
    let encoded_uri = urlencoding::encode(post_uri);
    let url = format!(
        "https://public.api.bsky.app/xrpc/app.bsky.feed.getLikes?uri={}&limit=100",
        encoded_uri
    );

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch likes: {}", e))?;

    if !response.status().is_success() {
        log::warn!(
            "Failed to fetch likes for {}: {}",
            post_uri,
            response.status()
        );
        return Ok((0, Vec::new()));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse likes response: {}", e))?;

    let mut likers = Vec::new();
    if let Some(likes) = json.get("likes").and_then(|l| l.as_array()) {
        for like in likes {
            if let (Some(did), Some(handle)) = (
                like.get("actor")
                    .and_then(|a| a.get("did"))
                    .and_then(|d| d.as_str()),
                like.get("actor")
                    .and_then(|a| a.get("handle"))
                    .and_then(|h| h.as_str()),
            ) {
                let display_name = like
                    .get("actor")
                    .and_then(|a| a.get("displayName"))
                    .and_then(|n| n.as_str())
                    .map(|s| s.to_string());

                likers.push(UserInfo {
                    did: did.to_string(),
                    handle: handle.to_string(),
                    display_name,
                });
            }
        }
    }

    let like_count = json
        .get("likes")
        .and_then(|l| l.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    Ok((like_count, likers))
}

/// Fetch reposts for a post from the AppView
async fn fetch_reposts(post_uri: &str) -> Result<(usize, Vec<UserInfo>), String> {
    let client = reqwest::Client::new();
    let encoded_uri = urlencoding::encode(post_uri);
    let url = format!(
        "https://public.api.bsky.app/xrpc/app.bsky.feed.getRepostedBy?uri={}&limit=100",
        encoded_uri
    );

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch reposts: {}", e))?;

    if !response.status().is_success() {
        log::warn!(
            "Failed to fetch reposts for {}: {}",
            post_uri,
            response.status()
        );
        return Ok((0, Vec::new()));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse reposts response: {}", e))?;

    let mut reposters = Vec::new();
    if let Some(users) = json.get("repostedBy").and_then(|r| r.as_array()) {
        for user in users {
            if let (Some(did), Some(handle)) = (
                user.get("did").and_then(|d| d.as_str()),
                user.get("handle").and_then(|h| h.as_str()),
            ) {
                let display_name = user
                    .get("displayName")
                    .and_then(|n| n.as_str())
                    .map(|s| s.to_string());

                reposters.push(UserInfo {
                    did: did.to_string(),
                    handle: handle.to_string(),
                    display_name,
                });
            }
        }
    }

    let repost_count = json
        .get("repostedBy")
        .and_then(|r| r.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    Ok((repost_count, reposters))
}

/// Extract image and video URLs from a post's embed field
async fn extract_media_urls(post: &AtRecord, did: &Did) -> (Vec<String>, Option<String>) {
    let mut image_urls = Vec::new();
    let mut video_url = None;

    // Get PDS endpoint for blob URLs
    let pds_endpoint = match resolve_did(did).await {
        Ok(pds) => pds,
        Err(_) => return (image_urls, video_url),
    };

    if let Some(embed) = post.value.get("embed") {
        let embed_type = embed.get("$type").and_then(|t| t.as_str());

        match embed_type {
            Some("app.bsky.embed.images") => {
                // Extract image CIDs
                if let Some(images) = embed.get("images").and_then(|i| i.as_array()) {
                    for img in images {
                        if let Some(cid) = img
                            .get("image")
                            .and_then(|i| i.get("ref"))
                            .and_then(|r| r.get("$link"))
                            .and_then(|l| l.as_str())
                        {
                            let url = format!(
                                "{}/xrpc/com.atproto.sync.getBlob?did={}&cid={}",
                                pds_endpoint,
                                did.as_str(),
                                cid
                            );
                            image_urls.push(url);
                        }
                    }
                }
            }
            Some("app.bsky.embed.video") => {
                // Extract video CID
                if let Some(cid) = embed
                    .get("video")
                    .and_then(|v| v.get("ref"))
                    .and_then(|r| r.get("$link"))
                    .and_then(|l| l.as_str())
                {
                    video_url = Some(format!(
                        "{}/xrpc/com.atproto.sync.getBlob?did={}&cid={}",
                        pds_endpoint,
                        did.as_str(),
                        cid
                    ));
                }
            }
            Some("app.bsky.embed.recordWithMedia") => {
                // Handle posts with both record and media
                if let Some(media) = embed.get("media") {
                    let media_type = media.get("$type").and_then(|t| t.as_str());
                    if media_type == Some("app.bsky.embed.images") {
                        if let Some(images) = media.get("images").and_then(|i| i.as_array()) {
                            for img in images {
                                if let Some(cid) = img
                                    .get("image")
                                    .and_then(|i| i.get("ref"))
                                    .and_then(|r| r.get("$link"))
                                    .and_then(|l| l.as_str())
                                {
                                    let url = format!(
                                        "{}/xrpc/com.atproto.sync.getBlob?did={}&cid={}",
                                        pds_endpoint,
                                        did.as_str(),
                                        cid
                                    );
                                    image_urls.push(url);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    (image_urls, video_url)
}

/// Analyze all posts from a user for labels and return both stats and labeled posts
pub async fn analyze_user_posts<F>(
    input: &str,
    auth_token: Option<String>,
    mut progress_callback: F,
) -> Result<(BulkAnalysisStats, Vec<PostWithLabels>), String>
where
    F: FnMut(String, u8),
{
    // Strip @ symbol if present (users might type @alice.bsky.social)
    let input = input.trim_start_matches('@');

    // Resolve handle to DID if needed
    let did = if input.starts_with("did:") {
        atproto_client::Did::new(input.to_string())
    } else {
        let handle = Handle::new(input.to_string());
        progress_callback("Resolving handle...".to_string(), 5);

        resolve_handle(&handle)
            .await
            .map_err(|e| format!("Failed to resolve handle: {}", e))?
    };

    // Fetch posts directly from PDS
    // Note: Banned/suspended accounts may be inaccessible
    progress_callback("Fetching posts from PDS...".to_string(), 15);
    let post_client = PostClient::new();
    let posts = post_client
        .fetch_posts(&did, 1000)
        .await
        .map_err(|e| format!("Failed to fetch posts: {}", e))?;

    progress_callback(
        format!("Fetched {} posts, querying labels...", posts.len()),
        20,
    );

    if posts.is_empty() {
        return Ok((
            BulkAnalysisStats {
                total_posts: 0,
                posts_with_labels: 0,
                labels_by_category: HashMap::new(),
                top_label_values: Vec::new(),
                account_labels: Vec::new(),
            },
            Vec::new(),
        ));
    }

    // Collect post URIs
    let uris: Vec<String> = posts.iter().map(|p| p.uri.clone()).collect();

    log::info!("Fetched {} posts from PDS. Sample URIs:", uris.len());
    for uri in uris.iter().take(3) {
        log::info!("  - {}", uri);
    }

    // Query labels from Bluesky's moderation service (including !takedown with auth)
    let batch_size = 25;
    let mut all_labels = Vec::new();
    let bsky_labeler = if let Some(token) = auth_token.clone() {
        log::info!(
            "Using authenticated labeler client (token: {}...)",
            &token[..20.min(token.len())]
        );
        LabelerClient::new_authenticated(token)
    } else {
        log::warn!("Using UNAUTHENTICATED labeler client - admin labels will NOT be visible!");
        LabelerClient::new()
    };

    // First, check for account-level labels on the DID itself
    progress_callback("Checking account-level labels...".to_string(), 25);
    log::info!("Querying account-level labels for DID: {}", did.as_str());

    let mut account_labels = Vec::new();
    match bsky_labeler.query_labels(&[did.as_str().to_string()]).await {
        Ok(collection) => {
            log::info!(
                "Account-level query returned {} labels",
                collection.labels.len()
            );
            for label in &collection.labels {
                log::info!("  Account Label: {} on {}", label.val, label.uri);
            }
            account_labels = collection.labels.clone();
            all_labels.extend(collection.labels);
        }
        Err(e) => {
            // Propagate authentication errors to the user
            if matches!(e, atproto_client::Error::AuthenticationRequired(_)) {
                return Err(e.to_string());
            }
            log::error!("Failed to query account-level labels: {}", e);
        }
    }

    let total_batches = uris.len().div_ceil(batch_size);
    for (i, chunk) in uris.chunks(batch_size).enumerate() {
        // Progress from 30% to 85% across all batches
        let batch_progress = 30 + ((i as f32 / total_batches as f32) * 55.0) as u8;
        progress_callback(
            format!(
                "Querying mod.bsky.app: batch {}/{}...",
                i + 1,
                total_batches
            ),
            batch_progress,
        );

        log::info!("Querying batch {} with {} URIs", i + 1, chunk.len());

        match bsky_labeler.query_labels(chunk).await {
            Ok(collection) => {
                log::info!(
                    "Batch {} returned {} labels",
                    i + 1,
                    collection.labels.len()
                );
                for label in &collection.labels {
                    log::info!("  Label: {} on {}", label.val, label.uri);
                }
                all_labels.extend(collection.labels);
            }
            Err(e) => {
                // Propagate authentication errors to the user
                if matches!(e, atproto_client::Error::AuthenticationRequired(_)) {
                    return Err(e.to_string());
                }
                log::error!("Failed to query mod.bsky.app batch {}: {}", i + 1, e);
            }
        }
    }

    log::info!(
        "Total labels found across all batches: {}",
        all_labels.len()
    );

    progress_callback("Analyzing results...".to_string(), 90);

    // Calculate statistics (only for post-level labels, not account labels)
    let mut posts_with_labels_set: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    let mut labels_by_category: HashMap<atproto_client::LabelCategory, usize> = HashMap::new();
    let mut label_value_counts: HashMap<String, usize> = HashMap::new();

    for label in &all_labels {
        if !label.neg {
            // Only count post URIs (not account-level DIDs)
            // Post URIs start with "at://" while DIDs start with "did:"
            if label.uri.starts_with("at://") {
                posts_with_labels_set.insert(label.uri.clone());

                let category = label.category();
                *labels_by_category.entry(category).or_insert(0) += 1;
                *label_value_counts.entry(label.val.clone()).or_insert(0) += 1;
            }
        }
    }

    // Sort label values by count
    let mut top_label_values: Vec<(String, usize)> = label_value_counts.into_iter().collect();
    top_label_values.sort_by(|a, b| b.1.cmp(&a.1));

    // Build posts with labels for display
    let mut labeled_posts = Vec::new();

    // If account has moderation labels (e.g., banned), show last 10 posts regardless of individual labels
    let has_account_moderation = !account_labels.is_empty();
    let mut posts_added = 0;
    let mut posts_processed = 0;

    for post in &posts {
        posts_processed += 1;
        // Update progress from 90% to 99% as we process posts
        if posts_processed % 100 == 0 || posts_processed == posts.len() {
            let process_progress = 90 + ((posts_processed as f32 / posts.len() as f32) * 9.0) as u8;
            progress_callback(
                format!("Processing posts ({}/{})...", posts_processed, posts.len()),
                process_progress,
            );
        }
        let post_labels: Vec<_> = all_labels
            .iter()
            .filter(|l| l.uri == post.uri)
            .cloned()
            .collect();

        // Show post if: has labels OR (account is moderated AND we haven't shown 10 yet)
        let should_show = !post_labels.is_empty() || (has_account_moderation && posts_added < 10);

        if should_show {
            let text = post
                .value
                .get("text")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();

            let created_at = post
                .value
                .get("createdAt")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();

            // Extract media URLs from embed
            let (image_urls, video_url) = extract_media_urls(post, &did).await;
            let has_media = !image_urls.is_empty() || video_url.is_some();

            // Fetch likes and reposts (especially useful for moderated posts)
            let (like_count, likers) = fetch_likes(&post.uri).await.unwrap_or((0, Vec::new()));
            let (repost_count, reposters) =
                fetch_reposts(&post.uri).await.unwrap_or((0, Vec::new()));

            log::info!(
                "Post {} has {} likes and {} reposts",
                post.uri,
                like_count,
                repost_count
            );

            labeled_posts.push(PostWithLabels {
                uri: post.uri.clone(),
                text,
                labels: post_labels,
                created_at,
                has_media,
                image_urls,
                video_url,
                like_count,
                repost_count,
                likers,
                reposters,
            });

            posts_added += 1;
        }
    }

    // Sort posts by number of labels (most labeled first), then by recency
    labeled_posts.sort_by(|a, b| {
        let label_cmp = b.labels.len().cmp(&a.labels.len());
        if label_cmp == std::cmp::Ordering::Equal {
            // If same number of labels, sort by created_at (most recent first)
            b.created_at.cmp(&a.created_at)
        } else {
            label_cmp
        }
    });

    progress_callback("Analysis complete!".to_string(), 100);

    Ok((
        BulkAnalysisStats {
            total_posts: posts.len(),
            posts_with_labels: posts_with_labels_set.len(),
            labels_by_category,
            top_label_values,
            account_labels,
        },
        labeled_posts,
    ))
}
