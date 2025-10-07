// SPDX-License-Identifier: MIT OR Apache-2.0

use leptos::*;
use wasm_bindgen_futures::spawn_local;

use crate::state::AppState;

#[component]
pub fn AuthPanel() -> impl IntoView {
    let state = expect_context::<AppState>();
    let show_auth = create_rw_signal(false);
    let handle = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let is_authenticating = create_rw_signal(false);
    
    let on_login = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        let handle_val = handle.get();
        let password_val = password.get();
        
        if handle_val.trim().is_empty() || password_val.trim().is_empty() {
            state.error.set(Some("Please enter both handle and app password".to_string()));
            return;
        }
        
        state.error.set(None);
        is_authenticating.set(true);
        
        spawn_local(async move {
            match crate::utils::authenticate(&handle_val, &password_val).await {
                Ok(token) => {
                    state.auth_token.set(Some(token));
                    state.is_authenticated.set(true);
                    state.error.set(None);
                    show_auth.set(false);
                    password.set(String::new()); // Clear password
                    log::info!("Successfully authenticated as {}", handle_val);
                }
                Err(e) => {
                    state.error.set(Some(format!("Login failed: {}", e)));
                    state.auth_token.set(None);
                    state.is_authenticated.set(false);
                }
            }
            is_authenticating.set(false);
        });
    };
    
    let on_logout = move |_| {
        state.auth_token.set(None);
        state.is_authenticated.set(false);
        handle.set(String::new());
        password.set(String::new());
        log::info!("Logged out");
    };
    
    view! {
        <div class="flex items-center gap-2">
            {move || if state.is_authenticated.get() {
                view! {
                    <div class="flex items-center gap-2">
                        <span class="text-sm text-green-600 dark:text-green-400">
                            "🔓 Authenticated"
                        </span>
                        <button
                            on:click=on_logout
                            class="px-3 py-1 text-sm bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 rounded transition-colors"
                        >
                            "Logout"
                        </button>
                    </div>
                }.into_view()
            } else {
                view! {
                    <button
                        on:click=move |_| show_auth.set(true)
                        class="px-3 py-2 text-sm bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors"
                    >
                        "🔐 Login"
                    </button>
                }.into_view()
            }}
        </div>
        
        {move || show_auth.get().then(|| view! {
            <div
                class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-50"
                on:click=move |_| show_auth.set(false)
            >
                <div
                    class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full p-6"
                    on:click=|ev| ev.stop_propagation()
                >
                    <div class="flex justify-between items-start mb-4">
                        <h3 class="text-xl font-bold text-gray-900 dark:text-gray-100">
                            "Login with App Password"
                        </h3>
                        <button
                            on:click=move |_| show_auth.set(false)
                            class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
                        >
                            "✕"
                        </button>
                    </div>
                    
                    <div class="mb-4 p-3 bg-blue-50 dark:bg-blue-900 rounded-lg">
                        <p class="text-sm text-blue-800 dark:text-blue-200">
                            "⚠️ Authentication is required to view admin labels like "
                            <code class="font-mono bg-blue-100 dark:bg-blue-800 px-1 rounded">"!takedown"</code>
                            ". Create an app password at: "
                            <a
                                href="https://bsky.app/settings/app-passwords"
                                target="_blank"
                                class="underline font-semibold"
                            >
                                "Settings → App Passwords"
                            </a>
                        </p>
                    </div>
                    
                    <form on:submit=on_login class="space-y-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                "Bluesky Handle"
                            </label>
                            <input
                                type="text"
                                placeholder="alice.bsky.social"
                                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent bg-white dark:bg-gray-700"
                                prop:value=move || handle.get()
                                on:input=move |ev| {
                                    handle.set(event_target_value(&ev));
                                }
                            />
                        </div>
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                "App Password"
                            </label>
                            <input
                                type="password"
                                placeholder="xxxx-xxxx-xxxx-xxxx"
                                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent bg-white dark:bg-gray-700"
                                prop:value=move || password.get()
                                on:input=move |ev| {
                                    password.set(event_target_value(&ev));
                                }
                            />
                        </div>
                        
                        <div class="text-xs text-gray-600 dark:text-gray-400">
                            "Your credentials are only used to get an access token and are not stored. The token is kept in your browser's memory for this session only."
                        </div>
                        
                        <button
                            type="submit"
                            disabled=move || is_authenticating.get()
                            class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
                        >
                            {move || if is_authenticating.get() {
                                "🔄 Authenticating..."
                            } else {
                                "🔐 Login"
                            }}
                        </button>
                    </form>
                </div>
            </div>
        })}
    }
}

