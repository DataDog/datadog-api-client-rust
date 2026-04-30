// Update a permanent retention filter returns "Updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters_permanent::RumRetentionFiltersPermanentAPI;
use datadog_api_client::datadogV2::model::RumPermanentCrossProductSamplingUpdate;
use datadog_api_client::datadogV2::model::RumPermanentRetentionFilterType;
use datadog_api_client::datadogV2::model::RumPermanentRetentionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::RumPermanentRetentionFilterUpdateData;
use datadog_api_client::datadogV2::model::RumPermanentRetentionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    let body =
        RumPermanentRetentionFilterUpdateRequest::new(RumPermanentRetentionFilterUpdateData::new(
            RumPermanentRetentionFilterUpdateAttributes::new().cross_product_sampling(
                RumPermanentCrossProductSamplingUpdate::new().trace_sample_rate(100.0 as f64),
            ),
            "default_replays".to_string(),
            RumPermanentRetentionFilterType::PERMANENT_RETENTION_FILTERS,
        ));
    let configuration = datadog::Configuration::new();
    let api = RumRetentionFiltersPermanentAPI::with_config(configuration);
    let resp = api
        .update_permanent_retention_filter(
            "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690".to_string(),
            "default_replays".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
