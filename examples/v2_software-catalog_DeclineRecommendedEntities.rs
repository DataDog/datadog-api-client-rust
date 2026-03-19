// Decline recommended entities in bulk returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_software_catalog::SoftwareCatalogAPI;
use datadog_api_client::datadogV2::model::RecommendedEntityID;

#[tokio::main]
async fn main() {
    let body = vec![RecommendedEntityID::new("123abcdef".to_string())];
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeclineRecommendedEntities", true);
    let api = SoftwareCatalogAPI::with_config(configuration);
    let resp = api.decline_recommended_entities(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
