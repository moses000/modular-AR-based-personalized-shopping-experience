use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to AR Shopping" }</h1>
            <button onclick={Callback::from(|_| log::info!("Detecting face..."))}>
                { "Scan Face" }
            </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

// 🔹 For WebAssembly, expose Rust functions via wasm/