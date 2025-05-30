use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let (stars, set_stars) = create_signal(0);
    
    // Function to handle the GitHub button click
    let open_github = move |_| {
        let window = web_sys::window().unwrap();
        let _ = window.open_with_url("https://github.com/fox7fog/nixos-cross-platform");
    };

    view! {
        <div class="page">
            // Hero Section
            <section class="hero">
                <h1>"NixOS Cross Platform"</h1>
                <p class="subtitle">
                    "A powerful cross-platform development environment using NixOS"
                </p>
                <div class="cta-buttons">
                    <button 
                        class="primary-button"
                        on:click=open_github
                    >
                        "View on GitHub"
                    </button>
                    <a 
                        href="/getting-started" 
                        class="secondary-button"
                    >
                        "Get Started"
                    </a>
                </div>
            </section>

            // Features Section
            <section class="features">
                <h2>"Key Features"</h2>
                <div class="feature-grid">
                    <div class="feature-card">
                        <h3>"Cross-Platform Support"</h3>
                        <p>"Seamlessly develop across different operating systems"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"NixOS Integration"</h3>
                        <p>"Leverage the power of NixOS package management"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"Developer Friendly"</h3>
                        <p>"Optimized for developer productivity and ease of use"</p>
                    </div>
                </div>
            </section>

            // Project Stats Section
            <section class="project-stats">
                <div class="stat-card">
                    <h3>{move || stars} " GitHub Stars"</h3>
                </div>
                <div class="stat-card">
                    <h3>"100% Open Source"</h3>
                </div>
            </section>
        </div>
    }
}