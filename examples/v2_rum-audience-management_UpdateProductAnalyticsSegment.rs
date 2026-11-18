// Update a segment returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;
use datadog_api_client::datadogV2::model::SegmentResourceType;
use datadog_api_client::datadogV2::model::SegmentUpdateAttributes;
use datadog_api_client::datadogV2::model::SegmentUpdateData;
use datadog_api_client::datadogV2::model::SegmentUpdateRequest;

#[tokio::main]
async fn main() {
    let body = SegmentUpdateRequest::new(SegmentUpdateData::new(
        SegmentUpdateAttributes::new()
            .description("Users who viewed the logs page in the last 7 days.".to_string())
            .name("High-value users".to_string()),
        "8a1b2c3d-4e5f-6a7b-8c9d-0e1f2a3b4c5d".to_string(),
        SegmentResourceType::SEGMENT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_product_analytics_segment", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api
        .update_product_analytics_segment("8a1b2c3d-4e5f-6a7b-8c9d-0e1f2a3b4c5d".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
