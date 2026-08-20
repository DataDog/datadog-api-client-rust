// Compute a Sankey diagram returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsAudienceAccountSubquery;
use datadog_api_client::datadogV2::model::ProductAnalyticsAudienceFilters;
use datadog_api_client::datadogV2::model::ProductAnalyticsAudienceSegmentSubquery;
use datadog_api_client::datadogV2::model::ProductAnalyticsAudienceUserSubquery;
use datadog_api_client::datadogV2::model::ProductAnalyticsJoinKeys;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyDefinition;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyRequestAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyRequestData;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyRequestType;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeySearch;
use datadog_api_client::datadogV2::model::ProductAnalyticsSankeyTime;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsSankeyRequest::new(ProductAnalyticsSankeyRequestData::new(
        ProductAnalyticsSankeyRequestAttributes::new(
            ProductAnalyticsSankeyDefinition::new(
                "@view.name".to_string(),
                "@view.name".to_string(),
            )
            .entries_per_step(10)
            .number_of_steps(3),
            ProductAnalyticsSankeySearch::new()
                .audience_filters(
                    ProductAnalyticsAudienceFilters::new()
                        .accounts(vec![ProductAnalyticsAudienceAccountSubquery::new(
                            "".to_string(),
                        )])
                        .formula("u".to_string())
                        .segments(vec![ProductAnalyticsAudienceSegmentSubquery::new(
                            "".to_string(),
                            Uuid::parse_str("00000000-0000-0000-0000-000000000000")
                                .expect("invalid UUID"),
                        )])
                        .users(vec![ProductAnalyticsAudienceUserSubquery::new(
                            "u".to_string(),
                        )
                        .query("*".to_string())]),
                )
                .join_keys(
                    ProductAnalyticsJoinKeys::new()
                        .primary("@session.id".to_string())
                        .secondary(vec![]),
                )
                .query("@type:view".to_string()),
            ProductAnalyticsSankeyTime::new(1756425600000, 1756857600000),
        ),
        ProductAnalyticsSankeyRequestType::SANKEY_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryProductAnalyticsSankey", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.query_product_analytics_sankey(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
