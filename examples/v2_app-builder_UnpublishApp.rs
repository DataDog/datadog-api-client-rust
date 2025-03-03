// Unpublish App returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;

#[tokio::main]
async fn main() {
    // there is a valid "app" in the system
    let app_data_id =
        uuid::Uuid::parse_str(&std::env::var("APP_DATA_ID").unwrap()).expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api.unpublish_app(app_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
