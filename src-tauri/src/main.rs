// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use url::form_urlencoded;
use models::{Host, HostResponse, AddressSource, BooleanFlag, ConnectType};


fn double_encode_special_characters_in_json(json: &str) -> String {
    let once_encoded = form_urlencoded::byte_serialize(json.as_bytes()).collect::<String>();

    let mut encoded_str = String::with_capacity(once_encoded.len());

    let mut chars = once_encoded.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            encoded_str.push('%');
            encoded_str.push_str(&chars.by_ref().take(2).collect::<String>());
        } else {
            encoded_str.push(ch);
        }
    }

    encoded_str
}

fn construct_payload(username: &str, password: &str) -> String {
    // Construct the raw JSON string
    let json_payload = format!(
        r#"{{"username":"{}", "password":"{}"}}"#,
        username, password
    );

    // URL-encode the entire JSON string and then double-encode special characters
    let encoded_payload = double_encode_special_characters_in_json(&json_payload);

    format!("model={}", encoded_payload)
}

async fn login_and_retrieve_cookie(
    client: &reqwest::Client,
    payload: &str,
) -> Result<String, reqwest::Error> {
    let login_url = "http://192.168.0.1/1/Device/Users/Login";
    let resp = client
        .post(login_url)
        .header(reqwest::header::ACCEPT, "*/*")
        .header(reqwest::header::CONNECTION, "keep-alive")
        .header(reqwest::header::ACCEPT_ENCODING, "gzip, deflate, br")
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(payload.to_string())
        .send()
        .await?;

    Ok(resp
        .headers()
        .get(reqwest::header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .split(';')
        .next()
        .unwrap_or("")
        .to_string())
}

async fn fetch_hosts_data(
    client: &reqwest::Client,
    cookie: &str,
) -> Result<HostResponse, Box<dyn Error>> {
    let hosts_url = "http://192.168.0.1/1/Device/Hosts";
    let cookie_header = format!(
        "{}; LANG_COOKIE=en_US; isEdit=0; isEdit1=0; isEdit2=0; isEdit3=0; modelname=CODA-4582-ROG",
        cookie
    );
    let resp = client
        .get(hosts_url)
        .header(reqwest::header::ACCEPT, "*/*")
        .header(reqwest::header::CONNECTION, "keep-alive")
        .header(reqwest::header::ACCEPT_ENCODING, "gzip, deflate, br")
        .header(reqwest::header::COOKIE, cookie_header)
        .send()
        .await?;

    let text_response = resp.text().await?;
    let hosts_data: HostResponse = serde_json::from_str(&text_response)?;

    Ok(hosts_data)
}

#[tauri::command]
async fn fetch_hosts() -> Result<HostResponse, String> {
    dotenv().ok();
    let username = env::var("ROUTER_USERNAME").unwrap_or_default();
    let password = env::var("ROUTER_PASSWORD").unwrap_or_default();

    let payload = construct_payload(username, password);

    let client = reqwest::Client::new();
    let cookie = login_and_retrieve_cookie(&client, &payload).await.map_err(|e| e.to_string())?;
    let mut hosts_data = fetch_hosts_data(&client, &cookie).await.map_err(|e| e.to_string())?;

    for host in &mut hosts_data.hosts_list {
        host.wifi_enabled = host.action == "Resume";
    }

    println!("Hosts data: ");
    println!("{:?}", hosts_data);

    Ok(hosts_data)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_hosts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
