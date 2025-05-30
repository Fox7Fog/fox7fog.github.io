use stylist::{style, yew::styled_component};
use yew::prelude::*;
use wasm_bindgen::prelude::*;

// Define the CSS for our application
fn get_styles() -> String {
    r#"
    /* Basic styling for the Rust WebAssembly GitHub Page */
    :root {
        --bg-light: #ffffff;
        --text-light: #1a1a1a;
        --primary-light: #006d77;
        --secondary-light: #f5f5f5;
        --border-light: #e5e5e5;

        --bg-dark: #1a1a1a;
        --text-dark: #ffffff;
        --primary-dark: #83c5be;
        --secondary-dark: #2d2d2d;
        --border-dark: #404040;
    }

    /* Theme styles */
    .light {
        --bg: var(--bg-light);
        --text: var(--text-light);
        --primary: var(--primary-light);
        --secondary: var(--secondary-light);
        --border: var(--border-light);
    }

    .dark {
        --bg: var(--bg-dark);
        --text: var(--text-dark);
        --primary: var(--primary-dark);
        --secondary: var(--secondary-dark);
        --border: var(--border-dark);
    }

    body {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
        margin: 0;
        padding: 0;
        background-color: var(--bg, #f5f5f5);
        color: var(--text, #333);
        line-height: 1.6;
    }

    #app {
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
        text-align: center;
    }

    h1 {
        color: var(--primary, #2c3e50);
        font-size: 2.5rem;
        margin-bottom: 1rem;
    }

    p {
        font-size: 1.2rem;
        margin-bottom: 1.5rem;
    }

    .content {
        background-color: white;
        border-radius: 8px;
        padding: 2rem;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        margin-top: 2rem;
    }

    /* Layout styles */
    .layout {
        display: flex;
        min-height: 100vh;
    }

    .sidebar {
        width: 280px;
        background-color: var(--secondary);
        border-right: 1px solid var(--border);
        padding: 1rem;
        position: fixed;
        height: 100vh;
        overflow-y: auto;
    }

    .sidebar-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    .logo {
        font-size: 1.25rem;
        font-weight: bold;
        color: var(--primary);
        text-decoration: none;
    }

    .theme-toggle {
        background: var(--secondary);
        border: 2px solid var(--primary);
        border-radius: 6px;
        cursor: pointer;
        padding: 0.5rem 1rem;
        font-size: 0.9rem;
        color: var(--primary);
        font-weight: 500;
        transition: all 0.2s ease;
    }

    .theme-toggle:hover {
        background: var(--primary);
        color: var(--bg);
        transform: translateY(-1px);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .main-content {
        margin-left: 280px;
        padding: 2rem;
        width: 100%;
        max-width: 1200px;
    }

    /* Hero Section */
    .hero {
        text-align: center;
        padding: 4rem 2rem;
        background: linear-gradient(135deg, var(--primary) 0%, #2a9d8f 100%);
        color: white;
        border-radius: 0 0 2rem 2rem;
    }

    .hero h1 {
        font-size: 3.5rem;
        margin-bottom: 1rem;
        font-weight: bold;
        color: white;
    }

    .hero .subtitle {
        font-size: 1.5rem;
        margin-bottom: 2rem;
        opacity: 0.9;
    }

    .cta-buttons {
        display: flex;
        gap: 1rem;
        justify-content: center;
        margin-top: 2rem;
    }

    .primary-button {
        background: white;
        color: var(--primary);
        padding: 0.8rem 1.6rem;
        border-radius: 0.5rem;
        font-weight: 600;
        border: none;
        cursor: pointer;
        transition: transform 0.2s;
    }

    .primary-button:hover {
        transform: translateY(-2px);
    }

    .secondary-button {
        background: transparent;
        color: white;
        padding: 0.8rem 1.6rem;
        border-radius: 0.5rem;
        font-weight: 600;
        border: 2px solid white;
        cursor: pointer;
        text-decoration: none;
        transition: background 0.2s;
    }

    .secondary-button:hover {
        background: rgba(255, 255, 255, 0.1);
    }

    /* Features Section */
    .features {
        padding: 4rem 2rem;
    }

    .features h2 {
        text-align: center;
        font-size: 2.5rem;
        margin-bottom: 3rem;
        color: var(--text);
    }

    .feature-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 2rem;
        max-width: 1200px;
        margin: 0 auto;
    }

    .feature-card {
        background: var(--secondary);
        padding: 2rem;
        border-radius: 1rem;
        text-align: center;
        transition: transform 0.2s;
    }

    .feature-card:hover {
        transform: translateY(-5px);
    }

    .feature-card h3 {
        color: var(--text);
        margin-bottom: 1rem;
        font-size: 1.5rem;
    }

    .feature-card p {
        color: var(--text);
        opacity: 0.8;
    }

    /* Responsive styles */
    @media (max-width: 768px) {
        .sidebar {
            transform: translateX(-100%);
            transition: transform 0.3s ease;
        }

        .sidebar.open {
            transform: translateX(0);
        }

        .main-content {
            margin-left: 0;
        }

        .hero h1 {
            font-size: 2.5rem;
        }

        .hero .subtitle {
            font-size: 1.2rem;
        }

        .cta-buttons {
            flex-direction: column;
            align-items: center;
        }

        .feature-grid {
            grid-template-columns: 1fr;
        }
    }
    "#.to_string()
}

// Define our app state
#[derive(Clone, PartialEq)]
pub struct AppState {
    theme: Theme,
}

#[derive(Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
        }
    }
}

// Define messages for our app
pub enum Msg {
    ToggleTheme,
}

// Define the App component
#[styled_component(App)]
pub fn app() -> Html {
    let state = use_state(AppState::default);

    // Create a callback to toggle the theme
    let toggle_theme = {
        let state = state.clone();
        Callback::from(move |_| {
            let current_theme = &state.theme;
            let new_theme = match current_theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
            state.set(AppState {
                theme: new_theme,
                ..(*state).clone()
            });
        })
    };

    // Determine the theme class
    let theme_class = match state.theme {
        Theme::Light => "light",
        Theme::Dark => "dark",
    };

    // Apply global styles
    let stylesheet = style! {
        get_styles()
    }.expect("Failed to parse stylesheet");

    html! {
        <>
            // Add the global stylesheet
            <div class={stylesheet}>
                // Set the theme class on the body
                <div class={classes!("layout", theme_class)}>
                    // Sidebar
                    <div class="sidebar">
                        <div class="sidebar-header">
                            <a href="#" class="logo">{"Rust WebAssembly"}</a>
                            <button class="theme-toggle" onclick={toggle_theme}>
                                {match state.theme {
                                    Theme::Light => "Dark Mode",
                                    Theme::Dark => "Light Mode",
                                }}
                            </button>
                        </div>
                        <nav class="nav-menu">
                            <ul>
                                <li><a href="#" class="nav-link">{"Home"}</a></li>
                                <li><a href="#features" class="nav-link">{"Features"}</a></li>
                                <li><a href="#about" class="nav-link">{"About"}</a></li>
                            </ul>
                            <div class="nav-section">
                                <span class="section-title">{"Resources"}</span>
                                <ul>
                                    <li><a href="https://www.rust-lang.org/" class="nav-link" target="_blank">{"Rust Language"}</a></li>
                                    <li><a href="https://webassembly.org/" class="nav-link" target="_blank">{"WebAssembly"}</a></li>
                                    <li><a href="https://yew.rs/" class="nav-link" target="_blank">{"Yew Framework"}</a></li>
                                </ul>
                            </div>
                        </nav>
                    </div>

                    // Main content
                    <div class="main-content">
                        // Hero section
                        <section class="hero">
                            <h1>{"Pure Rust Web Development"}</h1>
                            <div class="subtitle">{"Building modern web applications with Rust and WebAssembly"}</div>
                            <div class="cta-buttons">
                                <button class="primary-button">{"Get Started"}</button>
                                <a href="https://github.com/fox7fog/fox7fog.github.io" class="secondary-button" target="_blank">{"View on GitHub"}</a>
                            </div>
                        </section>

                        // Features section
                        <section id="features" class="features">
                            <h2>{"Features"}</h2>
                            <div class="feature-grid">
                                <div class="feature-card">
                                    <h3>{"100% Rust"}</h3>
                                    <p>{"This entire website is written in pure Rust, compiled to WebAssembly."}</p>
                                </div>
                                <div class="feature-card">
                                    <h3>{"Blazing Fast"}</h3>
                                    <p>{"WebAssembly provides near-native performance for web applications."}</p>
                                </div>
                                <div class="feature-card">
                                    <h3>{"Type Safety"}</h3>
                                    <p>{"Leverage Rust's powerful type system to build robust web applications."}</p>
                                </div>
                            </div>
                        </section>

                        // Project stats
                        <section class="project-stats">
                            <div class="stat-card">
                                <h3>{"100%"}</h3>
                                <p>{"Rust Code"}</p>
                            </div>
                            <div class="stat-card">
                                <h3>{"0%"}</h3>
                                <p>{"JavaScript"}</p>
                            </div>
                            <div class="stat-card">
                                <h3>{"∞"}</h3>
                                <p>{"Possibilities"}</p>
                            </div>
                        </section>
                    </div>
                </div>
            </div>
        </>
    }
}

// Keep these utility functions for backward compatibility
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
