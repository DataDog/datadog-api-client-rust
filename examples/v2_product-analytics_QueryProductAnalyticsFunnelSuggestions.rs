// Get feature suggestions returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsFunnelSuggestionsRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsFunnelSuggestionsRequestAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsFunnelSuggestionsRequestData;
use datadog_api_client::datadogV2::model::ProductAnalyticsFunnelSuggestionsRequestType;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsFunnelSuggestionsRequest::new(
        ProductAnalyticsFunnelSuggestionsRequestData::new(
            ProductAnalyticsFunnelSuggestionsRequestType::FUNNEL_SUGGESTIONS_REQUEST,
        )
        .attributes(
            ProductAnalyticsFunnelSuggestionsRequestAttributes::new()
                .application_id("40d8ca4b-2ac0-481f-b16e-f110699ecff6".to_string()),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.query_product_analytics_funnel_suggestions", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.query_product_analytics_funnel_suggestions(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
