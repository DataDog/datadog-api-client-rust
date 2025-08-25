// Get SPA Recommendations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_spa::SpaAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetSPARecommendations", true);
    let api = SpaAPI::with_config(configuration);
    let resp = api
        .get_spa_recommendations("shard".to_string(), "service".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
