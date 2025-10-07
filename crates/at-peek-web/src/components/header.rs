// SPDX-License-Identifier: MIT OR Apache-2.0

use leptos::*;
use super::AuthPanel;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
            <div class="container mx-auto px-4 py-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <span class="text-3xl">"🔍"</span>
                        <div>
                            <h1 class="text-2xl font-bold">
                                "at-peek"
                            </h1>
                            <p class="text-sm text-gray-600 dark:text-gray-400">
                                "ATproto Label Inspector"
                            </p>
                        </div>
                    </div>
                    
                    <div class="flex items-center gap-4">
                        <AuthPanel />
                        <a 
                            href="https://docs.bsky.app/docs/advanced-guides/moderation"
                            target="_blank"
                            class="text-sm text-blue-600 dark:text-blue-400 hover:underline"
                        >
                            "📚 Docs"
                        </a>
                    </div>
                </div>
            </div>
        </header>
    }
}


