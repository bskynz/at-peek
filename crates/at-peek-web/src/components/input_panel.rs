// SPDX-License-Identifier: MIT OR Apache-2.0

use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::state::AppState;
use crate::utils;

#[component]
pub fn InputPanel() -> impl IntoView {
    let state = expect_context::<AppState>();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let input = state.subject_input.get();

        if input.trim().is_empty() {
            state
                .error
                .set(Some("Please enter a handle, DID, or AT-URI".to_string()));
            return;
        }

        state.error.set(None);
        state.is_loading.set(true);

        spawn_local(async move {
            let auth_token = state.auth_token.get();
            let auth_did = state.authenticated_user_did.get();
            match utils::fetch_labels(&input, auth_token, auth_did).await {
                Ok(collection) => {
                    state.labels.set(Some(collection));
                    state.error.set(None);
                }
                Err(e) => {
                    state.error.set(Some(format!("Error: {}", e)));
                    state.labels.set(None);
                }
            }
            state.is_loading.set(false);
        });
    };

    view! {
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-6">
            <form on:submit=on_submit>
                <div class="mb-4">
                    <label
                        for="subject-input"
                        class="block text-sm font-medium mb-2"
                    >
                        "Enter Bluesky handle, DID, or post AT-URI"
                    </label>

                    <input
                        id="subject-input"
                        type="text"
                        placeholder="alice.bsky.social or did:plc:... or at://..."
                        class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent bg-white dark:bg-gray-700"
                        prop:value=move || state.subject_input.get()
                        on:input=move |ev| {
                            state.subject_input.set(event_target_value(&ev));
                        }
                    />

                    <p class="mt-2 text-xs text-gray-500 dark:text-gray-400">
                        "Examples: alice.bsky.social • did:plc:xyz123 • at://did:plc:xyz/app.bsky.feed.post/abc"
                    </p>
                </div>

                <button
                    type="submit"
                    disabled=move || state.is_loading.get()
                    class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
                >
                    {move || if state.is_loading.get() {
                        "🔄 Checking Labels..."
                    } else {
                        "🔍 Check Labels"
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
                                        "Some content requires authentication. Private accounts and certain moderation labels (like "
                                        <code class="bg-yellow-200 dark:bg-yellow-800 px-1 rounded">"!ban"</code>
                                        ", "
                                        <code class="bg-yellow-200 dark:bg-yellow-800 px-1 rounded">"!takedown"</code>
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

            {move || state.error.get().map(|err| view! {
                <div class="mt-4 p-4 bg-red-100 dark:bg-red-900 border border-red-300 dark:border-red-700 rounded-lg">
                    <p class="text-red-800 dark:text-red-200 text-sm">
                        {err}
                    </p>
                </div>
            })}
        </div>
    }
}
