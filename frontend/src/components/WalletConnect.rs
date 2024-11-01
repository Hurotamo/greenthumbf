use yew::prelude::*;

#[function_component(WalletConnect)]
pub fn wallet_connect() -> Html {
    let connect_wallet = Callback::from(|_| {
        // Logic for connecting to Solana wallet (Phantom/Solflare)
        web_sys::console::log_1(&"Wallet connected".into());
    });

    html! {
        <button onclick={connect_wallet} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
            {"Connect Wallet"}
        </button>
    }
}

