// Create a Product Analytics monitor returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorCreateAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorCreateData;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorCreateRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorOptions;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorRequestType;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorThresholds;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsMonitorCreateRequest::new(ProductAnalyticsMonitorCreateData::new(
        ProductAnalyticsMonitorCreateAttributes::new(
            r#"Conversion from /a to /b dropped over the last hour

@slack-my-alerts-channel"#
                .to_string(),
            "Funnel conversion drop".to_string(),
            r#"formula("query").rollup("sum").last("1h") < 1"#.to_string(),
        )
        .options(
            ProductAnalyticsMonitorOptions::new()
                .thresholds(
                    ProductAnalyticsMonitorThresholds::new()
                        .critical(1.0 as f64)
                        .warning(2.0 as f64),
                )
                .variables(vec![BTreeMap::from([
                    (
                        "data_source".to_string(),
                        Value::from("product_analytics_journey"),
                    ),
                    ("name".to_string(), Value::from("query")),
                ])]),
        )
        .priority(3)
        .tags(BTreeMap::from([
            ("chart_id".to_string(), "chart-1".to_string()),
            ("query_type".to_string(), "funnel".to_string()),
        ])),
        ProductAnalyticsMonitorRequestType::PA_MONITOR_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_product_analytics_monitor", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.create_product_analytics_monitor(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
