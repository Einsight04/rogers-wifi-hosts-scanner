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

#[function_component(App)]
pub fn app() -> Html {
    let hosts = use_state(Vec::<Host>::new);

    {
        let fetch_and_update_hosts = {
            let hosts = hosts.clone();
            move || {
                spawn_local(async move {
                    let response = invoke("fetch_hosts", JsValue::NULL).await;
                    if let Ok(host_response) = serde_wasm_bindgen::from_value::<HostResponse>(response) {
                        hosts.set(host_response.hosts_list);
                    }
                });
            }
        };

        let hosts_clone = hosts.clone();

        use_effect_with_deps(
            move |_| {
                fetch_and_update_hosts();
                || {}
            },
            hosts_clone,
        );
    }


    html! {
        <main class="container">
            <div>
                <h2>{"Hosts:"}</h2>
                <ul>
                {
                    for hosts.iter().map(|host| html! {
                        <li>
                            <span>{ &host.host_name }</span>
                            <span>{ &host.ip }</span>
                            <span>{ host.wifi_enabled.to_string() }</span>
                        </li>
                    })
                }
                </ul>
            </div>
        </main>
    }
}
