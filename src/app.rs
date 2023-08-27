use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use models::{Host, HostResponse};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let hosts = use_state(Vec::new);

    let fetch_and_update_hosts = {
        let hosts = hosts.clone();
        move || {
            spawn_local(async move {
                let response: JsValue = invoke("fetch_hosts", JsValue::NULL).await;
                if let Ok(host_response) = serde_wasm_bindgen::from_value::<HostResponse>(response) {
                    hosts.set(host_response.hosts_list);
                }
            });
        }
    };

    use_effect(|| {
        fetch_and_update_hosts();
    });

    html! {
        <main class="container">
            <div>
                <h2>{"Hosts:"}</h2>
                <ul>
                    { for hosts.iter().map(|host| html! { <li>{ &host.host_name }</li> }) }
                </ul>
                { for (0..3).map(|_| html! { <p>{"Static Host"}</p> }) }
            </div>
        </main>
    }
}
