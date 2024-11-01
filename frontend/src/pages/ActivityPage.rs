use yew::prelude::*;

#[function_component(ActivityPage)]
pub fn activity_page() -> Html {
    html! {
        <div class="activity-page">
            <CameraSetup />
        </div>
    }
}

