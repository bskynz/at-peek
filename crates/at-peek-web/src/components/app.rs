// SPDX-License-Identifier: MIT OR Apache-2.0

use leptos::*;

#[allow(unused_imports)]
use leptos_meta::*;

use super::{BulkAnalysis, Header, InputPanel, LabelViewer};
use crate::state::AppState;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let state = AppState::new();
    let mode = create_rw_signal("single"); // "single" or "bulk"

    provide_context(state);

    view! {
        <Stylesheet id="leptos" href="/pkg/at-peek-web.css"/>
        <Link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css"/>

        <Title text="at-peek - ATproto Label Inspector"/>
        <Meta name="description" content="Inspect content moderation labels on ATproto users and posts"/>

        <div class="min-h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
            <Header />

            <main class="container mx-auto px-4 py-8 max-w-4xl">
                <div class="mb-6 flex gap-2">
                    <button
                        class=move || format!(
                            "px-4 py-2 rounded-lg font-semibold transition-colors {}",
                            if mode.get() == "single" {
                                "bg-blue-600 text-white"
                            } else {
                                "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                            }
                        )
                        on:click=move |_| mode.set("single")
                    >
                        "🔍 Single Check"
                    </button>
                    <button
                        class=move || format!(
                            "px-4 py-2 rounded-lg font-semibold transition-colors {}",
                            if mode.get() == "bulk" {
                                "bg-blue-600 text-white"
                            } else {
                                "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                            }
                        )
                        on:click=move |_| mode.set("bulk")
                    >
                        "📊 Bulk Analysis"
                    </button>
                </div>

                <Show
                    when=move || mode.get() == "single"
                    fallback=|| view! { <BulkAnalysis /> }
                >
                    <InputPanel />
                    <LabelViewer />
                </Show>
            </main>

            <footer class="mt-16 py-8 border-t border-gray-200 dark:border-gray-700">
                <div class="container mx-auto px-4 text-center text-sm text-gray-600 dark:text-gray-400">
                    <p>
                        "at-peek v0.1.0 is open source • "
                        <a href="https://github.com/bskynz/at-peek" target="_blank" class="text-blue-600 dark:text-blue-400 hover:underline">
                            "GitHub"
                        </a>
                    </p>
                    <p class="mt-2 text-xs">
                        "🔒 All processing happens locally in your browser. No data leaves your device."
                    </p>
                    <p class="mt-1 text-xs text-gray-500 dark:text-gray-500">
                        "Note: This site is hosted on Cloudflare Pages. Cloudflare terminates TLS and can technically access data in transit, including authentication credentials. Use app passwords, not your main account password."
                    </p>
                </div>
            </footer>
        </div>
    }
}
