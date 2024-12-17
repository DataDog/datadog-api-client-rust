// List Apps returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_apps::AppsAPI;
use datadog_api_client::datadogV2::api_apps::ListAppsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListApps", true);
    let api = AppsAPI::with_config(configuration);
    let resp = api.list_apps(ListAppsOptionalParams::default()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
