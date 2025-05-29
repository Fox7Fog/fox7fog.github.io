use leptos::*;

#[component]
pub fn GettingStarted(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="page">
            <h1>"Getting Started"</h1>
            <p>"Welcome to the documentation. This guide will help you get started with the project."</p>
        </div>
    }
}
