// SPDX-License-Identifier: MIT OR Apache-2.0

use leptos::prelude::*;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;

use crate::state::AppState;
use atproto_client::LabelCategory;

#[derive(Clone, Debug)]
pub struct BulkAnalysisStats {
    pub total_posts: usize,
    pub posts_with_labels: usize,
    pub labels_by_category: HashMap<LabelCategory, usize>,
    pub top_label_values: Vec<(String, usize)>,
    pub account_labels: Vec<atproto_client::Label>,
}

#[derive(Clone, Debug)]
pub struct PostWithLabels {
    pub uri: String,
    pub text: String,
    pub labels: Vec<atproto_client::Label>,
    pub created_at: String,
    pub has_media: bool,
    pub image_urls: Vec<String>,
    pub video_url: Option<String>,
    pub like_count: usize,
    pub repost_count: usize,
    pub likers: Vec<UserInfo>,
    pub reposters: Vec<UserInfo>,
}

#[derive(Clone, Debug)]
pub struct UserInfo {
    #[allow(dead_code)]
    pub did: String,
    pub handle: String,
    pub display_name: Option<String>,
}

#[component]
pub fn BulkAnalysis() -> impl IntoView {
    let state = expect_context::<AppState>();
    let stats = create_rw_signal::<Option<BulkAnalysisStats>>(None);
    let labeled_posts = create_rw_signal::<Vec<PostWithLabels>>(Vec::new());
    let selected_post = create_rw_signal::<Option<PostWithLabels>>(None);
    let is_analyzing = create_rw_signal(false);
    let progress = create_rw_signal::<Option<String>>(None);
    let progress_percent = create_rw_signal(0);

    let on_analyze = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let input = state.subject_input.get();

        if input.trim().is_empty() {
            state
                .error
                .set(Some("Please enter a Bluesky handle".to_string()));
            return;
        }

        state.error.set(None);
        stats.set(None);
        labeled_posts.set(Vec::new());
        is_analyzing.set(true);
        progress.set(Some("Starting analysis...".to_string()));
        progress_percent.set(0);

        spawn_local(async move {
            let auth_token = state.auth_token.get();
            match crate::utils::analyze_user_posts(&input, auth_token, |msg, percent| {
                progress.set(Some(msg));
                progress_percent.set(percent);
            })
            .await
            {
                Ok((analysis_stats, posts)) => {
                    stats.set(Some(analysis_stats));
                    labeled_posts.set(posts);
                    state.error.set(None);
                    progress.set(None);
                }
                Err(e) => {
                    state.error.set(Some(format!("Error: {}", e)));
                    stats.set(None);
                    labeled_posts.set(Vec::new());
                    progress.set(None);
                }
            }
            is_analyzing.set(false);
        });
    };

    view! {
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-6">
            <h2 class="text-xl font-bold mb-4">
                "📊 Bulk Post Analysis"
            </h2>
            <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
                "Analyze the last 1000 posts from a user to see label statistics"
            </p>

            <form on:submit=on_analyze>
                <div class="mb-4">
                    <input
                        type="text"
                        placeholder="Enter Bluesky handle (e.g., alice.bsky.social)"
                        class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent bg-white dark:bg-gray-700"
                        prop:value=move || state.subject_input.get()
                        on:input=move |ev| {
                            state.subject_input.set(event_target_value(&ev));
                        }
                    />
                </div>

                <button
                    type="submit"
                    disabled=move || is_analyzing.get()
                    class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
                >
                    {move || if is_analyzing.get() {
                        "🔄 Analyzing..."
                    } else {
                        "📊 Analyze Last 1000 Posts"
                    }}
                </button>
            </form>

            {move || {
                if state.auth_token.get().is_none() {
                    Some(view! {
                        <div class="mt-4 p-4 bg-yellow-100 dark:bg-yellow-900 border-l-4 border-yellow-500 rounded-r-lg">
                            <div class="flex items-start gap-3">
                                <div class="text-xl">"⚠️"</div>
                                <div class="flex-1">
                                    <p class="text-sm font-semibold text-yellow-900 dark:text-yellow-100 mb-1">
                                        "Not Authenticated"
                                    </p>
                                    <p class="text-sm text-yellow-800 dark:text-yellow-200">
                                        "Some moderation actions (like "
                                        <code class="bg-yellow-200 dark:bg-yellow-800 px-1 rounded">"!ban"</code>
                                        ", "
                                        <code class="bg-yellow-200 dark:bg-yellow-800 px-1 rounded">"!takedown"</code>
                                        ", "
                                        <code class="bg-yellow-200 dark:bg-yellow-800 px-1 rounded">"!hide"</code>
                                        ") will not be visible. Sign in with your Bluesky account to see all labels."
                                    </p>
                                </div>
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}

            {move || progress.get().map(|msg| {
                let percent = progress_percent.get();
                view! {
                    <div class="mt-4 p-4 bg-blue-50 dark:bg-blue-900 rounded-lg border border-blue-200 dark:border-blue-700">
                        <div class="flex items-center justify-between mb-2">
                            <p class="text-sm font-semibold text-blue-800 dark:text-blue-200">
                                {msg}
                            </p>
                            <span class="text-sm font-bold text-blue-600 dark:text-blue-400">
                                {percent}"%"
                            </span>
                        </div>
                        <div class="w-full bg-blue-200 dark:bg-blue-800 rounded-full h-2.5 overflow-hidden">
                            <div
                                class="bg-blue-600 dark:bg-blue-400 h-2.5 rounded-full transition-all duration-300 ease-out"
                                style=move || format!("width: {}%", progress_percent.get())
                            />
                        </div>
                    </div>
                }
            })}

            {move || stats.get().map(|s| view! {
                <div class="mt-6">
                    <StatsDisplay stats=s />
                </div>
            })}

            {move || {
                let posts = labeled_posts.get();
                if !posts.is_empty() {
                    let has_account_labels = stats.get().map(|s| !s.account_labels.is_empty()).unwrap_or(false);
                    Some(view! {
                        <div class="mt-6">
                            <LabeledPostsList posts=posts selected_post=selected_post has_account_labels=has_account_labels />
                        </div>
                    })
                } else {
                    None
                }
            }}

            {move || selected_post.get().map(|post| view! {
                <PostDetailModal post=post on_close=move || selected_post.set(None) />
            })}
        </div>
    }
}

#[component]
fn StatsDisplay(stats: BulkAnalysisStats) -> impl IntoView {
    let percentage_with_labels = if stats.total_posts > 0 {
        stats.posts_with_labels as f64 / stats.total_posts as f64 * 100.0
    } else {
        0.0
    };

    view! {
        <div class="space-y-4">
            <h3 class="text-lg font-bold">"Analysis Results"</h3>

            {(!stats.account_labels.is_empty()).then(|| view! {
                <div class="p-4 bg-red-100 dark:bg-red-900 border-2 border-red-500 rounded-lg">
                    <div class="flex items-start gap-3">
                        <div class="text-2xl">"⚠️"</div>
                        <div class="flex-1">
                            <h4 class="font-bold text-red-900 dark:text-red-100 mb-2">
                                "Account-Level Moderation Labels"
                            </h4>
                            <p class="text-sm text-red-800 dark:text-red-200 mb-3">
                                "This account has been flagged by moderators. Posts are shown for transparency but may violate community guidelines."
                            </p>
                            <div class="flex flex-wrap gap-2">
                                {stats.account_labels.iter().map(|label| {
                                view! {
                                    <span class="px-3 py-1 bg-red-200 dark:bg-red-800 text-red-900 dark:text-red-100 rounded-full text-sm font-mono">
                                        {label.val.clone()}
                                    </span>
                                }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>
                </div>
            })}

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                    <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">
                        {stats.total_posts}
                    </div>
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        "Total Posts Analyzed"
                    </div>
                </div>

                <div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                    <div class="text-2xl font-bold text-orange-600 dark:text-orange-400">
                        {stats.posts_with_labels}
                    </div>
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        "Posts with Labels"
                    </div>
                </div>

                <div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                    <div class="text-2xl font-bold text-red-600 dark:text-red-400">
                        {format!("{:.1}%", percentage_with_labels)}
                    </div>
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        "Labeled Posts"
                    </div>
                </div>
            </div>

                {{
                    let categories = stats.labels_by_category.clone();
                    let total = stats.total_posts as f64;
                    let categories_for_show = categories.clone();
                    view! {
                        <Show
                            when=move || !categories_for_show.is_empty()
                            fallback=|| view! {
                                <div>
                                    <div class="p-4 bg-green-100 dark:bg-green-900 rounded-lg">
                                        <p class="text-green-800 dark:text-green-200">
                                            "✅ No moderation labels found on any posts!"
                                        </p>
                                    </div>
                                </div>
                            }
                        >
                            <div>
                                <h4 class="text-md font-bold mb-3">"Labels by Category"</h4>
                                <div class="space-y-2">
                                    {categories.iter().map(|(category, count)| {
                                        let pct = *count as f64 / total * 100.0;
                                        let count_val = *count;
                                        let cat = category.clone();

                                        view! {
                                            <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
                                                <div class="flex items-center gap-2">
                                                    <span class="text-xl">{cat.icon()}</span>
                                                    <span class="font-medium">{cat.name()}</span>
                                                </div>
                                                <div class="text-right">
                                                    <span class="font-bold">{count_val}</span>
                                                    <span class="text-sm text-gray-600 dark:text-gray-400 ml-2">
                                                        {format!("({:.1}%)", pct)}
                                                    </span>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        </Show>
                    }
                }}

            {{
                let top_labels = stats.top_label_values.clone();
                let top_labels_for_show = top_labels.clone();
                view! {
                    <Show
                        when=move || !top_labels_for_show.is_empty()
                        fallback=|| view! { <div></div> }
                    >
                        <div>
                            <h4 class="text-md font-bold mb-3">"Top Label Types"</h4>
                            <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
                                {top_labels.iter().take(6).map(|(label, count)| {
                                    let label_val = label.clone();
                                    let count_val = *count;

                                    view! {
                                        <div class="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
                                            <div class="font-mono text-sm font-bold">{label_val}</div>
                                            <div class="text-xs text-gray-600 dark:text-gray-400">
                                                {format!("{} occurrence{}", count_val, if count_val == 1 { "" } else { "s" })}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </Show>
                }
            }}
        </div>
    }
}

#[component]
fn LabeledPostsList(
    posts: Vec<PostWithLabels>,
    selected_post: RwSignal<Option<PostWithLabels>>,
    has_account_labels: bool,
) -> impl IntoView {
    // Count how many posts actually have labels
    let posts_with_actual_labels = posts.iter().filter(|p| !p.labels.is_empty()).count();

    // Determine the appropriate header
    let (header_text, subtitle) = if has_account_labels && posts_with_actual_labels == 0 {
        (
            format!("📝 Recent Posts from Moderated Account ({})", posts.len()),
            Some(
                "Showing recent posts for transparency. Individual posts may not have labels."
                    .to_string(),
            ),
        )
    } else if has_account_labels {
        (
            format!("📝 Posts from Moderated Account ({})", posts.len()),
            Some(format!(
                "{} post{} {} individual labels",
                posts_with_actual_labels,
                if posts_with_actual_labels == 1 {
                    ""
                } else {
                    "s"
                },
                if posts_with_actual_labels == 1 {
                    "has"
                } else {
                    "have"
                }
            )),
        )
    } else {
        (format!("📝 Posts with Labels ({})", posts.len()), None)
    };

    view! {
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
            <h3 class="text-lg font-bold mb-2">
                {header_text}
            </h3>
            {subtitle.map(|text| view! {
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
                    {text}
                </p>
            })}
            <div class="space-y-2 max-h-96 overflow-y-auto">
                <For
                    each=move || posts.clone()
                    key=|post| post.uri.clone()
                    children=move |post: PostWithLabels| {
                        let post_clone = post.clone();
                        view! {
                            <div
                                class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 cursor-pointer transition-colors"
                                on:click=move |_| selected_post.set(Some(post_clone.clone()))
                            >
                                <div class="flex items-start justify-between">
                                    <div class="flex-1 min-w-0">
                                        <p class="text-sm text-gray-900 dark:text-gray-100 truncate mb-2">
                                            {if post.text.is_empty() {
                                                "[No text]".to_string()
                                            } else if post.text.len() > 100 {
                                                format!("{}...", &post.text[..100])
                                            } else {
                                                post.text.clone()
                                            }}
                                        </p>
                                        <div class="flex flex-wrap gap-1 mb-1">
                                            {post.labels.iter().map(|label| {
                                                let category = label.category();
                                                let color = match category {
                                                    LabelCategory::AdultContent => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
                                                    LabelCategory::Violence => "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200",
                                                    LabelCategory::Spam => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
                                                    LabelCategory::Hate => "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
                                                    LabelCategory::ModerationAction => "bg-pink-100 text-pink-800 dark:bg-pink-900 dark:text-pink-200",
                                                    _ => "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200",
                                                };
                                view! {
                                    <span class=format!("px-2 py-1 rounded text-xs font-medium {}", color)>
                                        {label.val.clone()}
                                    </span>
                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                        <p class="text-xs text-gray-500 dark:text-gray-400">
                                            {crate::utils::format_timestamp(&post.created_at)}
                                            {if post.has_media { " • 📎 Has media" } else { "" }}
                                        </p>
                                        <div class="flex gap-3 mt-2 text-xs text-gray-600 dark:text-gray-400">
                                            <span>{"❤️ "}{post.like_count}{" likes"}</span>
                                            <span>{"🔁 "}{post.repost_count}{" reposts"}</span>
                                        </div>
                                    </div>
                                    <div class="ml-4 flex-shrink-0">
                                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">
                                            {post.labels.len()} " label" {if post.labels.len() == 1 { "" } else { "s" }}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn PostDetailModal<F>(post: PostWithLabels, on_close: F) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    let show_likers = create_rw_signal(false);
    let show_reposters = create_rw_signal(false);

    // Clone the post fields to avoid move issues
    let post_created_at = post.created_at.clone();
    let post_image_urls = post.image_urls.clone();
    let post_labels = post.labels.clone();

    view! {
        <div
            class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-50"
            on:click=move |_| on_close()
        >
            <div
                class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full max-h-[80vh] overflow-y-auto"
                on:click=|ev| ev.stop_propagation()
            >
                <div class="p-6">
                    <div class="flex justify-between items-start mb-4">
                        <h3 class="text-xl font-bold text-gray-900 dark:text-gray-100">
                            "Post Details"
                        </h3>
                        <button
                            on:click=move |_| on_close()
                            class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
                        >
                            "✕"
                        </button>
                    </div>

                    <div class="space-y-4">
                        <div>
                            <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
                                "Content"
                            </h4>
                            <p class="text-gray-900 dark:text-gray-100 whitespace-pre-wrap">
                                {if post.text.is_empty() { "[No text content]".to_string() } else { post.text.clone() }}
                            </p>
                        </div>

                        <div>
                            <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
                                "Labels (" {post.labels.len()} ")"
                            </h4>
                            <div class="space-y-2">
                                {post_labels.iter().map(|label| {
                                    let category = label.category();
                                    view! {
                                        <div class="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
                                            <div class="flex items-center justify-between mb-2">
                                                <span class="font-mono text-sm font-semibold">
                                                    {category.icon()} {" "} {label.val.clone()}
                                                </span>
                                                <span class="text-xs text-gray-500 dark:text-gray-400">
                                                    {category.name()}
                                                </span>
                                            </div>
                                            <div class="text-xs text-gray-600 dark:text-gray-400 space-y-1">
                                                <div>"Source: " <span class="font-mono">{crate::utils::shorten_did(&label.src)}</span></div>
                                                <div>"Applied: " {crate::utils::format_timestamp(&label.cts)}</div>
                                                {{
                                                    let created_at = post_created_at.clone();
                                                    if !created_at.is_empty() {
                                                        let duration = crate::utils::calculate_duration(&created_at, &label.cts);
                                                        let is_moderation = label.val.starts_with('!');
                                                        let color = if is_moderation {
                                                            "text-pink-600 dark:text-pink-400 font-semibold"
                                                        } else {
                                                            "text-gray-600 dark:text-gray-400"
                                                        };
                                                        view! {
                                                            <div class=format!("flex items-center gap-1 {}", color)>
                                                                <span>{"⏱️ "}</span>
                                                                <span>{duration}</span>
                                                                <span>{" after post"}</span>
                                                            </div>
                                                        }
                                                    } else {
                                                        view! {
                                                            <div class=format!("flex items-center gap-1 {}", "text-gray-600 dark:text-gray-400")>
                                                                <span>{"⏱️ "}</span>
                                                                <span>{"No timestamp available".to_string()}</span>
                                                                <span>{" after post"}</span>
                                                            </div>
                                                        }
                                                    }
                                                }}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>

                        <div>
                            <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
                                "URI"
                            </h4>
                            <a
                                href=format!("https://bsky.app/profile/{}", post.uri.replace("at://", "").replace("/app.bsky.feed.post/", "/post/"))
                                target="_blank"
                                class="text-blue-600 dark:text-blue-400 hover:underline text-sm font-mono break-all"
                            >
                                {post.uri.clone()}
                            </a>
                        </div>

                        <div>
                            <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
                                "Posted"
                            </h4>
                            <p class="text-sm text-gray-600 dark:text-gray-400">
                                {crate::utils::format_timestamp(&post.created_at)}
                            </p>
                        </div>

                        // Display images
                        {{
                            let image_urls = post_image_urls.clone();
                            let image_urls_for_show = image_urls.clone();
                            view! {
                                <Show
                                    when=move || !image_urls_for_show.is_empty()
                                    fallback=|| view! { <div></div> }
                                >
                                    <div>
                                        <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
                                            "Images"
                                        </h4>
                                        <div class="grid grid-cols-2 gap-2">
                                            {image_urls.iter().map(|url| {
                                        let url_clone = url.clone();
                                        view! {
                                            <img
                                                src=url_clone.clone()
                                                class="w-full rounded border border-gray-300 dark:border-gray-600 cursor-pointer hover:opacity-80"
                                                alt="Post image"
                                                on:click=move |_| {
                                                    // Open in new tab
                                                    if let Some(window) = web_sys::window() {
                                                        let _ = window.open_with_url_and_target(&url_clone, "_blank");
                                                    }
                                                }
                                            />
                                        }
                                    }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                </Show>
                            }
                        }}

                        // Display video
                        {move || {
                            post.video_url.as_ref().map(|video_url| {
                                let video_url_clone = video_url.clone();
                                view! {
                                    <div>
                                        <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
                                            "Video"
                                        </h4>
                                        <video
                                            src=video_url_clone
                                            controls=true
                                            class="w-full rounded border border-gray-300 dark:border-gray-600"
                                        >
                                            "Your browser does not support the video tag."
                                        </video>
                                    </div>
                                }
                            })
                        }}

                        // Display likes (expandable) - Always show, even with zero
                        {
                            let likers_clone = post.likers.clone();
                            let like_count = post.like_count;
                            view! {
                                <div class="border-t border-gray-200 dark:border-gray-700 pt-3">
                                    <button
                                        class="w-full flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors"
                                        on:click=move |e| {
                                            e.stop_propagation();
                                            if like_count > 0 {
                                                show_likers.update(|v| *v = !*v);
                                            }
                                        }
                                        disabled=like_count == 0
                                    >
                                        <span class="text-sm font-semibold">
                                            "❤️ " {like_count} " Like" {if like_count == 1 { "" } else { "s" }}
                                        </span>
                                        {view! {
                                                <span class="text-xs">
                                      {move || if like_count > 0 {
                                          if show_likers.get() { "▼" } else { "▶" }
                                      } else {
                                          ""
                                      }}
                                  </span>
                      }}
                            </button>

                            {move || {
                                if show_likers.get() && like_count > 0 {
                                    Some(view! {
                                        <div class="mt-2 max-h-48 overflow-y-auto space-y-1">
                                            {likers_clone.iter().map(|liker| {
                                                let display = if let Some(name) = &liker.display_name {
                                                    format!("{} (@{})", name, liker.handle)
                                                } else {
                                                    format!("@{}", liker.handle)
                                                };
                                                view! {
                                                    <a
                                                        href=format!("https://bsky.app/profile/{}", liker.handle)
                                                        target="_blank"
                                                        class="block p-2 bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700 text-sm"
                                                    >
                                                        {display}
                                                    </a>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    })
                                } else {
                                    None
                                }
                            }}
                                </div>
                            }
                        }

                        // Display reposts (expandable) - Always show, even with zero
                        {
                            let reposters_clone = post.reposters.clone();
                            let repost_count = post.repost_count;
                            view! {
                                <div class="border-t border-gray-200 dark:border-gray-700 pt-3">
                                    <button
                                        class="w-full flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors"
                                        on:click=move |e| {
                                            e.stop_propagation();
                                            if repost_count > 0 {
                                                show_reposters.update(|v| *v = !*v);
                                            }
                                        }
                                        disabled=repost_count == 0
                                    >
                                        <span class="text-sm font-semibold">
                                            "🔁 " {repost_count} " Repost" {if repost_count == 1 { "" } else { "s" }}
                                        </span>
                                        {view! {
                                                <span class="text-xs">
                                      {move || if repost_count > 0 {
                                          if show_reposters.get() { "▼" } else { "▶" }
                                      } else {
                                          ""
                                      }}
                                  </span>
                      }}
                            </button>

                            {move || {
                                if show_reposters.get() && repost_count > 0 {
                                    Some(view! {
                                        <div class="mt-2 max-h-48 overflow-y-auto space-y-1">
                                            {reposters_clone.iter().map(|reposter| {
                                                let display = if let Some(name) = &reposter.display_name {
                                                    format!("{} (@{})", name, reposter.handle)
                                                } else {
                                                    format!("@{}", reposter.handle)
                                                };
                                                view! {
                                                    <a
                                                        href=format!("https://bsky.app/profile/{}", reposter.handle)
                                                        target="_blank"
                                                        class="block p-2 bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700 text-sm"
                                                    >
                                                        {display}
                                                    </a>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    })
                                } else {
                                    None
                                }
                            }}
                                </div>
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
