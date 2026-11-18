// Preview a segment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;
use datadog_api_client::datadogV2::model::SegmentDataQuery;
use datadog_api_client::datadogV2::model::SegmentEventPlatformBlock;
use datadog_api_client::datadogV2::model::SegmentPreviewAttributes;
use datadog_api_client::datadogV2::model::SegmentPreviewData;
use datadog_api_client::datadogV2::model::SegmentPreviewRequest;
use datadog_api_client::datadogV2::model::SegmentPreviewRequestType;

#[tokio::main]
async fn main() {
    let body = SegmentPreviewRequest::new(SegmentPreviewData::new(
        SegmentPreviewAttributes::new(SegmentDataQuery::new().event_platform(vec![
                            SegmentEventPlatformBlock::new("@usr.id".to_string())
                                .from("-2592000000".to_string())
                                .query("@type:view @view.name:/logs".to_string())
                                .to("0".to_string())
                        ]))
        .limit(50)
        .user_store_columns(vec![
            "user_id".to_string(),
            "user_email".to_string(),
            "user_name".to_string(),
        ]),
        SegmentPreviewRequestType::SEGMENT_PREVIEW_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.preview_product_analytics_segment", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api.preview_product_analytics_segment(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
