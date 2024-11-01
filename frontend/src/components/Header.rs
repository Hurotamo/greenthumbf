use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="flex justify-between items-center p-4 bg-green-600 text-white">
            <h1 class="text-xl font-bold">{"GreenThumb"}</h1>
            <div class="wallet-connect">
                <WalletConnect />
            </div>
        </header>
    }
}

