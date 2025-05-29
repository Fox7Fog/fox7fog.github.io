use crate::data::navigation::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar(
    sidebar_open: ReadSignal<bool>,
    set_sidebar_open: WriteSignal<bool>,
    search_query: ReadSignal<String>,
    set_search_query: WriteSignal<String>,
) -> impl IntoView {
    let nav_items = get_navigation_items();

    view! {
        <aside class=move || format!(
            "fixed inset-y-0 left-0 z-40 w-80 bg-gray-800 border-r border-gray-700 transform transition-transform duration-300 ease-in-out lg:translate-x-0 {}",
            if sidebar_open.get() { "translate-x-0" } else { "-translate-x-full" }
        )>
            <div class="flex flex-col h-full">
                // Header
                <div class="flex items-center justify-between p-4 border-b border-gray-700">
                    <h1 class="text-xl font-bold text-white">"Your Portfolio"</h1>
                    <button
                        class="lg:hidden p-1 rounded text-gray-400 hover:text-white"
                        on:click=move |_| set_sidebar_open.set(false)
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </button>
                </div>

                // Search
                <div class="p-4 border-b border-gray-700">
                    <input
                        type="text"
                        placeholder="Search documentation..."
                        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        prop:value=move || search_query.get()
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                    />
                </div>

                // Navigation
                <nav class="flex-1 overflow-y-auto p-4">
                    <ul class="space-y-2">
                        {nav_items.into_iter().map(|section| view! {
                            <NavSection section=section search_query=search_query/>
                        }).collect::<Vec<_>>()}
                    </ul>
                </nav>

                // Footer
                <div class="p-4 border-t border-gray-700">
                    <div class="flex space-x-4 text-sm text-gray-400">
                        <a href="https://github.com/yourusername" class="hover:text-white">
                            "GitHub"
                        </a>
                        <a href="mailto:your@email.com" class="hover:text-white">
                            "Contact"
                        </a>
                    </div>
                </div>
            </div>
        </aside>

        // Overlay for mobile
        <Show when=move || sidebar_open.get()>
            <div
                class="fixed inset-0 z-30 bg-black bg-opacity-50 lg:hidden"
                on:click=move |_| set_sidebar_open.set(false)
            ></div>
        </Show>
    }
}

#[component]
fn NavSection(section: NavigationSection, search_query: ReadSignal<String>) -> impl IntoView {
    let (expanded, set_expanded) = create_signal(section.expanded);

    view! {
        <li>
            <button
                class="flex items-center justify-between w-full px-2 py-1 text-left text-gray-300 hover:text-white hover:bg-gray-700 rounded"
                on:click=move |_| set_expanded.update(|exp| *exp = !*exp)
            >
                <span class="font-medium">{section.title}</span>
                <svg
                    class=move || format!("w-4 h-4 transform transition-transform {}",
                        if expanded.get() { "rotate-90" } else { "" })
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                </svg>
            </button>

            <Show when=move || expanded.get()>
                <ul class="ml-4 mt-1 space-y-1">
                    {section.items.into_iter().map(|item| view! {
                        <NavItem item=item search_query=search_query/>
                    }).collect::<Vec<_>>()}
                </ul>
            </Show>
        </li>
    }
}

#[component]
fn NavItem(item: NavigationItem, search_query: ReadSignal<String>) -> impl IntoView {
    let is_visible = move || {
        let query = search_query.get().to_lowercase();
        query.is_empty() || item.title.to_lowercase().contains(&query)
    };

    view! {
        <Show when=is_visible>
            <li>
                <A
                    href=item.path
                    class="block px-2 py-1 text-sm text-gray-400 hover:text-white hover:bg-gray-700 rounded"
                    active_class="text-blue-400 bg-gray-700"
                >
                    {item.title}
                </A>
            </li>
        </Show>
    }
}
