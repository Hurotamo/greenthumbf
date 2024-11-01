use yew::prelude::*;

mod components;
mod pages;

use components::{Header};
use pages::{HomePage, ProfilePage, MarketplacePage, ActivityPage};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="p-4">
                <HomePage />
            </main>
        </>
    }
}

