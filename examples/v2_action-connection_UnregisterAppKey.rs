// Unregister an App Key returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api
        .unregister_app_key("57cc69ae-9214-4ecc-8df8-43ecc1d92d99".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
