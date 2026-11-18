// Create a segment returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;
use datadog_api_client::datadogV2::model::SegmentCreateAttributes;
use datadog_api_client::datadogV2::model::SegmentCreateData;
use datadog_api_client::datadogV2::model::SegmentCreateRequest;
use datadog_api_client::datadogV2::model::SegmentDataQuery;
use datadog_api_client::datadogV2::model::SegmentEventPlatformBlock;
use datadog_api_client::datadogV2::model::SegmentResourceType;

#[tokio::main]
async fn main() {
    let body = SegmentCreateRequest::new(SegmentCreateData::new(
        SegmentCreateAttributes::new(
            SegmentDataQuery::new().event_platform(vec![SegmentEventPlatformBlock::new(
                "@usr.id".to_string(),
            )
            .from("-2592000000".to_string())
            .name("viewed_logs".to_string())
            .query("@type:view @view.name:/logs".to_string())
            .to("0".to_string())]),
            "High-value users".to_string(),
        )
        .description("Users who viewed the logs page in the last 30 days.".to_string())
        .tags(vec!["team:growth".to_string()]),
        SegmentResourceType::SEGMENT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_product_analytics_segment", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api.create_product_analytics_segment(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
