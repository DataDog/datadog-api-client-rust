// Get SPA recommendations v2 returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_spa::SpaAPI;
use datadog_api_client::datadogV2::model::RecommendationV2RequestAttributes;
use datadog_api_client::datadogV2::model::RecommendationV2RequestBody;
use datadog_api_client::datadogV2::model::RecommendationV2RequestData;
use datadog_api_client::datadogV2::model::RecommendationV2RequestType;

#[tokio::main]
async fn main() {
    let body = RecommendationV2RequestBody::new(RecommendationV2RequestData::new(
        RecommendationV2RequestAttributes::new(vec!["".to_string()]),
        RecommendationV2RequestType::RECOMMENDATION_V2_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_spa_recommendations_v2", true);
    let api = SpaAPI::with_config(configuration);
    let resp = api
        .get_spa_recommendations_v2("service".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
