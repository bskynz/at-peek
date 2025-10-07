// SPDX-License-Identifier: MIT OR Apache-2.0

use leptos::prelude::*;

#[component]
pub fn EmptyState() -> impl IntoView {
    view! {
        <div class="text-center py-12">
            <div class="text-6xl mb-4">"✅"</div>
            <h3 class="text-xl font-bold mb-2">
                "No moderation labels found"
            </h3>
            <p class="text-gray-600 dark:text-gray-400">
                "This user or post has no labels applied by moderation services."
            </p>
        </div>
    }
}
