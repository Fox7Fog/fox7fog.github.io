mod home;
mod getting_started;
mod cheatsheets;
mod projects;

pub use home::Home;
pub use getting_started::GettingStarted;
pub use cheatsheets::*;
pub use projects::*;

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="page">
            <h1>"404 - Page Not Found"</h1>
            <p>"The page you're looking for doesn't exist."</p>
        </div>
    }
}