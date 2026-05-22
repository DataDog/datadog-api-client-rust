// Delete an entity integration configuration returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_entity_integration_configs::EntityIntegrationConfigsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteEntityIntegrationConfig", true);
    let api = EntityIntegrationConfigsAPI::with_config(configuration);
    let resp = api
        .delete_entity_integration_config("github".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
