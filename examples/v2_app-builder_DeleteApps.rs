// Delete Multiple Apps returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::AppDefinitionType;
use datadog_api_client::datadogV2::model::DeleteAppsRequest;
use datadog_api_client::datadogV2::model::DeleteAppsRequestDataItems;

#[tokio::main]
async fn main() {
    // there is a valid "app" in the system
    let app_data_id =
        uuid::Uuid::parse_str(&std::env::var("APP_DATA_ID").unwrap()).expect("Invalid UUID");
    let body = DeleteAppsRequest::new().data(vec![DeleteAppsRequestDataItems::new(
        app_data_id.clone(),
        AppDefinitionType::APPDEFINITIONS,
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteApps", true);
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api.delete_apps(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
