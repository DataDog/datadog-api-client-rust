// Compute Sankey flow analysis returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsJoinKeys;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyDefinition;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyRequestAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyRequestData;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyRequestType;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeySearch;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyTime;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsSankeyRequest::new(ProductAnalyticsSankeyRequestData::new(
        ProductAnalyticsSankeyRequestAttributes::new(
            "product_analytics".to_string(),
            ProductAnalyticsSankeyDefinition::new()
                .entries_per_step(5)
                .number_of_steps(5)
                .source("/logs".to_string())
                .target("".to_string()),
            ProductAnalyticsSankeySearch::new("@type:view".to_string())
                .join_keys(ProductAnalyticsJoinKeys::new().primary("@session.id".to_string())),
            ProductAnalyticsSankeyTime::new(1771232048460, 1771836848262),
        ),
        ProductAnalyticsSankeyRequestType::SANKEY_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.query_product_analytics_sankey(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
