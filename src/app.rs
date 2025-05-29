use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{layout::*, theme::*};
use crate::pages::*;
use crate::data::*;

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

                    <Route path="/*any" view=NotFound/>
                </Routes>
            </MainLayout>
        </Router>
    }
}
