// Update a hardcoded retention filter returns "Updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters_hardcoded::RUMRetentionFiltersHardcodedAPI;
use datadog_api_client::datadogV2::model::RumHardcodedCrossProductSamplingUpdate;
use datadog_api_client::datadogV2::model::RumHardcodedRetentionFilterType;
use datadog_api_client::datadogV2::model::RumHardcodedRetentionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::RumHardcodedRetentionFilterUpdateData;
use datadog_api_client::datadogV2::model::RumHardcodedRetentionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    let body =
        RumHardcodedRetentionFilterUpdateRequest::new(RumHardcodedRetentionFilterUpdateData::new(
            RumHardcodedRetentionFilterUpdateAttributes::new().cross_product_sampling(
                RumHardcodedCrossProductSamplingUpdate::new()
                    .session_replay_enabled(true)
                    .session_replay_sample_rate(50.0 as f64),
            ),
            "REPLACE.ME".to_string(),
            RumHardcodedRetentionFilterType::HARDCODED_RETENTION_FILTERS,
        ));
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionFiltersHardcodedAPI::with_config(configuration);
    let resp = api
        .update_hardcoded_retention_filter(
            "Example-RUM-Retention-Filters-Hardcoded".to_string(),
            "Example-RUM-Retention-Filters-Hardcoded".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
