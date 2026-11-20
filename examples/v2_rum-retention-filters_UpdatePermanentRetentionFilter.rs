// Update a permanent RUM retention filter returns "Updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RUMRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RUMCrossProductSamplingUpdate;
use datadog_api_client::datadogV2::model::RUMPermanentRetentionFilterID;
use datadog_api_client::datadogV2::model::RUMPermanentRetentionFilterType;
use datadog_api_client::datadogV2::model::RUMPermanentRetentionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::RUMPermanentRetentionFilterUpdateData;
use datadog_api_client::datadogV2::model::RUMPermanentRetentionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    let body =
        RUMPermanentRetentionFilterUpdateRequest::new(RUMPermanentRetentionFilterUpdateData::new(
            RUMPermanentRetentionFilterUpdateAttributes::new().cross_product_sampling(
                RUMCrossProductSamplingUpdate::new()
                    .trace_enabled(true)
                    .trace_sample_rate(25.0 as f64),
            ),
            RUMPermanentRetentionFilterID::SYNTHETICS_SESSIONS,
            RUMPermanentRetentionFilterType::PERMANENT_RETENTION_FILTERS,
        ));
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .update_permanent_retention_filter(
            "app_id".to_string(),
            RUMPermanentRetentionFilterID::SYNTHETICS_SESSIONS,
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
