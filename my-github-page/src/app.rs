use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;
mod data;

use components::{layout::*, theme::*};
use pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Title text="Rust Documentation"/>
        <Meta name="description" content="Comprehensive Rust documentation and guides"/>
        <Link rel="stylesheet" href="/src/styles/main.css"/>
        
        <Router>
            <MainLayout>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/getting-started" view=GettingStarted/>
                    <Route path="/guides">
                        <Route path="/installation" view=Installation/>
                        <Route path="/configuration" view=Configuration/>
                    </Route>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </MainLayout>
        </Router>
    }
}
