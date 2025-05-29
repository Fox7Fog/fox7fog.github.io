mod home;
mod getting_started;
mod cheatsheets;
mod projects;

pub use home::Home;
pub use getting_started::GettingStarted;
pub use cheatsheets::*;
pub use projects::*;

use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="page">
            <h1>"404 - Page Not Found"</h1>
            <p>"The page you're looking for doesn't exist."</p>
        </div>
    }
}