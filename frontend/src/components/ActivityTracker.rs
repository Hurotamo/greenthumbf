use yew::prelude::*;

#[function_component(ActivityTracker)]
pub fn activity_tracker() -> Html {
    html! {
        <div class="activity-tracker">
            {"Track user activities like watering, planting, and harvesting."}
        </div>
    }
}

