use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <h2>{"Welcome to GreenThumb"}</h2>
            <ActivityTracker />
        </div>
    }
}

