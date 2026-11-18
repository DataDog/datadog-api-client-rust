// Get funnel step suggestions returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsFunnelStepSuggestionsRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsFunnelStepSuggestionsRequestAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsFunnelStepSuggestionsRequestData;
use datadog_api_client::datadogV2::model::ProductAnalyticsFunnelStepSuggestionsRequestType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsFunnelStepSuggestionsRequest::new(
        ProductAnalyticsFunnelStepSuggestionsRequestData::new(
            ProductAnalyticsFunnelStepSuggestionsRequestAttributes::new(
                "node_01->node_02".to_string(),
                BTreeMap::from([
                    (
                        "node_01".to_string(),
                        "@type:view @view.name:/checkout".to_string(),
                    ),
                    (
                        "node_02".to_string(),
                        "@type:action @action.name:pay".to_string(),
                    ),
                ]),
            )
            .application_id("40d8ca4b-2ac0-481f-b16e-f110699ecff6".to_string()),
            ProductAnalyticsFunnelStepSuggestionsRequestType::FUNNEL_SUGGESTIONS_STEPS_REQUEST,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.query_product_analytics_funnel_step_suggestions", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api
        .query_product_analytics_funnel_step_suggestions(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
