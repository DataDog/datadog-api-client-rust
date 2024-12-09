// Delete Multiple Apps returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_apps::AppsAPI;
use datadog_api_client::datadogV2::model::DeleteAppsRequest;
use datadog_api_client::datadogV2::model::DeleteAppsRequestDataItems;
use datadog_api_client::datadogV2::model::DeleteAppsRequestDataItemsType;

#[tokio::main]
async fn main() {
    // there is a valid "app" in the system
    let app_data_id = std::env::var("APP_DATA_ID").unwrap();
    let body = DeleteAppsRequest::new().data(vec![DeleteAppsRequestDataItems::new(
        app_data_id.clone(),
        DeleteAppsRequestDataItemsType::APPDEFINITIONS,
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteApps", true);
    let api = AppsAPI::with_config(configuration);
    let resp = api.delete_apps(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
