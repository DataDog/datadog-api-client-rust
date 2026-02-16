// Trigger recommended entity discovery returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_software_catalog::SoftwareCatalogAPI;
use datadog_api_client::datadogV2::api_software_catalog::TriggerRecommendedEntitiesOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.TriggerRecommendedEntities", true);
    let api = SoftwareCatalogAPI::with_config(configuration);
    let resp = api
        .trigger_recommended_entities(TriggerRecommendedEntitiesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
