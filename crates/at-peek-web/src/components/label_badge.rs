// SPDX-License-Identifier: MIT OR Apache-2.0

use atproto_client::Label;
use leptos::prelude::*;

use crate::utils;

#[component]
pub fn LabelBadge(label: Label) -> impl IntoView {
    let category = label.category();
    let color_class = match category {
        atproto_client::LabelCategory::AdultContent => {
            "bg-red-100 dark:bg-red-900 border-red-300 dark:border-red-700"
        }
        atproto_client::LabelCategory::Violence => {
            "bg-orange-100 dark:bg-orange-900 border-orange-300 dark:border-orange-700"
        }
        atproto_client::LabelCategory::Spam => {
            "bg-yellow-100 dark:bg-yellow-900 border-yellow-300 dark:border-yellow-700"
        }
        atproto_client::LabelCategory::Hate => {
            "bg-red-200 dark:bg-red-800 border-red-400 dark:border-red-600"
        }
        atproto_client::LabelCategory::ModerationAction => {
            "bg-blue-100 dark:bg-blue-900 border-blue-300 dark:border-blue-700"
        }
        atproto_client::LabelCategory::Other => {
            "bg-gray-100 dark:bg-gray-700 border-gray-300 dark:border-gray-600"
        }
    };

    let formatted_time = utils::format_timestamp(&label.cts);
    let shortened_did = utils::shorten_did(&label.src);

    view! {
        <div
            class=format!("p-4 rounded-lg border-2 {} transition-all hover:shadow-md", color_class)
            title=label.description()
        >
            <div class="flex items-start justify-between">
                <div class="flex-1">
                    <div class="flex items-center gap-2 mb-2">
                        <span class="text-2xl">{category.icon()}</span>
                        <span class="font-bold text-lg">{label.val.clone()}</span>
                    </div>

                    <p class="text-sm opacity-75 mb-2">
                        {label.description()}
                    </p>

                    <div class="text-xs opacity-60 space-y-1">
                        <div>
                            <span class="font-semibold">"Source: "</span>
                            <span>{shortened_did}</span>
                        </div>
                        <div>
                            <span class="font-semibold">"Created: "</span>
                            <span>{formatted_time}</span>
                        </div>
                        {label.exp.as_ref().map(|exp| {
                            let formatted_exp = utils::format_timestamp(exp);
                            view! {
                                <div>
                                    <span class="font-semibold">"Expires: "</span>
                                    <span>{formatted_exp}</span>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </div>
        </div>
    }
}
