use leptos::*;
use web_sys::Storage;

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let (theme, set_theme) = create_signal(Theme::Dark);
    
    // Initialize theme from localStorage
    let window = web_sys::window().unwrap();
    let storage: Storage = window.local_storage().unwrap().unwrap();
    
    if let Ok(Some(saved_theme)) = storage.get_item("theme") {
        if saved_theme == "light" {
            set_theme(Theme::Light);
        }
    }
    
    // Update body class and localStorage when theme changes
    create_effect(move |_| {
        let current_theme = theme();
        let _ = storage.set_item("theme", current_theme.as_str());
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        
        match current_theme {
            Theme::Light => {
                body.class_list().remove_1("dark").unwrap();
                body.class_list().add_1("light").unwrap();
            }
            Theme::Dark => {
                body.class_list().remove_1("light").unwrap();
                body.class_list().add_1("dark").unwrap();
            }
        }
    });

    provide_context((theme, set_theme));
    
    view! {
        <div class="theme-provider">
            {children()}
        </div>
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = use_context::<(ReadSignal<Theme>, WriteSignal<Theme>)>()
        .expect("ThemeToggle must be used within ThemeProvider");

    let toggle_theme = move |_| {
        set_theme.update(|t| {
            *t = match t {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            }
        });
    };

    view! {
        <button
            class="theme-toggle"
            on:click=toggle_theme
            aria-label="Toggle theme"
        >
            {move || match theme() {
                Theme::Light => "Switch to dark mode",
                Theme::Dark => "Switch to light mode",
            }}
        </button>
    }
}
