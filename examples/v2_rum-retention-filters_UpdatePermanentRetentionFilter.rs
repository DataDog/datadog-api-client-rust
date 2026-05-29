// Update a permanent RUM retention filter returns "Updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RumRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RumCrossProductSamplingUpdate;
use datadog_api_client::datadogV2::model::RumPermanentRetentionFilterID;
use datadog_api_client::datadogV2::model::RumPermanentRetentionFilterType;
use datadog_api_client::datadogV2::model::RumPermanentRetentionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::RumPermanentRetentionFilterUpdateData;
use datadog_api_client::datadogV2::model::RumPermanentRetentionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    let body =
        RumPermanentRetentionFilterUpdateRequest::new(RumPermanentRetentionFilterUpdateData::new(
            RumPermanentRetentionFilterUpdateAttributes::new().cross_product_sampling(
                RumCrossProductSamplingUpdate::new()
                    .trace_enabled(true)
                    .trace_sample_rate(25.0 as f64),
            ),
            RumPermanentRetentionFilterID::SYNTHETICS_SESSIONS,
            RumPermanentRetentionFilterType::PERMANENT_RETENTION_FILTERS,
        ));
    let configuration = datadog::Configuration::new();
    let api = RumRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .update_permanent_retention_filter(
            "app_id".to_string(),
            RumPermanentRetentionFilterID::SYNTHETICS_SESSIONS,
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
