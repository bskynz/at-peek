// SPDX-License-Identifier: MIT OR Apache-2.0

use leptos::*;
use atproto_client::LabelCategory;

use crate::state::AppState;
use super::{LabelBadge, EmptyState};

#[component]
pub fn LabelViewer() -> impl IntoView {
    let state = expect_context::<AppState>();
    
    let categorized_labels = move || {
        state.labels.get().map(|collection| {
            let mut categories: std::collections::HashMap<LabelCategory, Vec<_>> = std::collections::HashMap::new();
            
            for label in collection.labels {
                categories.entry(label.category())
                    .or_default()
                    .push(label);
            }
            
            categories
        })
    };
    
    view! {
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
            <Show
                when=move || state.labels.get().is_some()
                fallback=|| view! { 
                    <div class="text-center py-12 text-gray-500 dark:text-gray-400">
                        "Enter a subject above to check for moderation labels"
                    </div>
                }
            >
                {move || {
                    let Some(categories) = categorized_labels() else {
                        return view! { <div/> }.into_view();
                    };
                    
                    if categories.is_empty() {
                        return view! { <EmptyState /> }.into_view();
                    }
                    
                    view! {
                        <div class="space-y-6">
                            <h2 class="text-xl font-bold mb-4">
                                "🏷️ Moderation Labels Found"
                            </h2>
                            
                            <For
                                each=move || {
                                    let mut cats: Vec<_> = categories.iter()
                                        .map(|(k, v)| (k.clone(), v.clone()))
                                        .collect();
                                    cats.sort_by_key(|(cat, _)| format!("{:?}", cat));
                                    cats
                                }
                                key=|(cat, _)| format!("{:?}", cat)
                                let:item
                            >
                                <CategoryGroup category=item.0 labels=item.1 />
                            </For>
                        </div>
                    }.into_view()
                }}
            </Show>
        </div>
    }
}

#[component]
fn CategoryGroup(category: LabelCategory, labels: Vec<atproto_client::Label>) -> impl IntoView {
    let expanded = create_rw_signal(true);
    let labels = create_rw_signal(labels);
    
    view! {
        <div class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
            <div 
                class="flex items-center justify-between p-4 cursor-pointer bg-gray-50 dark:bg-gray-750 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                on:click=move |_| expanded.update(|e| *e = !*e)
            >
                <div class="flex items-center gap-3">
                    <span class="text-2xl">{category.icon()}</span>
                    <span class="font-semibold text-lg">{category.name()}</span>
                    <span class="px-2 py-1 bg-gray-200 dark:bg-gray-600 rounded-full text-sm">
                        {move || labels.get().len()}
                    </span>
                </div>
                <span class="text-gray-500">
                    {move || if expanded.get() { "▼" } else { "▶" }}
                </span>
            </div>
            
            <Show when=move || expanded.get()>
                <div class="p-4 space-y-3 bg-white dark:bg-gray-800">
                    <For
                        each=move || labels.get()
                        key=|label| format!("{}:{}:{}", label.val, label.src, label.cts)
                        let:label
                    >
                        <LabelBadge label=label />
                    </For>
                </div>
            </Show>
        </div>
    }
}


