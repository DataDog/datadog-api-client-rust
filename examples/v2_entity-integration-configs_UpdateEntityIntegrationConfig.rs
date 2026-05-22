// Create or update entity integration configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_entity_integration_configs::EntityIntegrationConfigsAPI;
use datadog_api_client::datadogV2::model::EntityIntegrationConfigRequest;
use datadog_api_client::datadogV2::model::EntityIntegrationConfigRequestAttributes;
use datadog_api_client::datadogV2::model::EntityIntegrationConfigRequestData;
use datadog_api_client::datadogV2::model::EntityIntegrationConfigRequestType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = EntityIntegrationConfigRequest::new(EntityIntegrationConfigRequestData::new(
        EntityIntegrationConfigRequestAttributes::new(BTreeMap::from([])),
        EntityIntegrationConfigRequestType::ENTITY_INTEGRATION_CONFIG_REQUESTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateEntityIntegrationConfig", true);
    let api = EntityIntegrationConfigsAPI::with_config(configuration);
    let resp = api
        .update_entity_integration_config("github".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
