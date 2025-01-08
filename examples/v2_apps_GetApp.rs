// Get App returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_apps::AppsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "app" in the system
    let app_data_id = std::env::var("APP_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetApp", true);
    let api = AppsAPI::with_config(configuration);
    let resp = api.get_app(app_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
