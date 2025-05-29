use leptos::*;
use leptos_router::*;
use super::theme::*;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <ThemeProvider>
            <div class="layout">
                <nav class="sidebar">
                    <div class="sidebar-header">
                        <A href="/" class="logo">
                            "Documentation"
                        </A>
                        <ThemeToggle/>
                    </div>
                    <NavMenu/>
                </nav>
                <main class="main-content">
                    {children()}
                </main>
            </div>
        </ThemeProvider>
    }
}

#[component]
fn NavMenu() -> impl IntoView {
    view! {
        <nav class="nav-menu">
            <ul>
                <li>
                    <A href="/" class="nav-link">
                        "Home"
                    </A>
                </li>
                <li>
                    <A href="/getting-started" class="nav-link">
                        "Getting Started"
                    </A>
                </li>
                <li class="nav-section">
                    <span class="section-title">"Guides"</span>
                    <ul>
                        <li>
                            <A href="/guides/installation" class="nav-link">
                                "Installation"
                            </A>
                        </li>
                        <li>
                            <A href="/guides/configuration" class="nav-link">
                                "Configuration"
                            </A>
                        </li>
                    </ul>
                </li>
            </ul>
        </nav>
    }
}
