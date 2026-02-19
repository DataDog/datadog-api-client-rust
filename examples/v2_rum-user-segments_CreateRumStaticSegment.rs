// Create a static RUM segment returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_user_segments::RUMUserSegmentsAPI;
use datadog_api_client::datadogV2::model::RumStaticSegmentCreateAttributes;
use datadog_api_client::datadogV2::model::RumStaticSegmentCreateData;
use datadog_api_client::datadogV2::model::RumStaticSegmentCreateRequest;
use datadog_api_client::datadogV2::model::RumStaticSegmentJourneyFilter;
use datadog_api_client::datadogV2::model::RumStaticSegmentJourneyNode;
use datadog_api_client::datadogV2::model::RumStaticSegmentJourneyQueryObject;
use datadog_api_client::datadogV2::model::RumStaticSegmentRequestType;

#[tokio::main]
async fn main() {
    let body = RumStaticSegmentCreateRequest::new(RumStaticSegmentCreateData::new(
        RumStaticSegmentCreateAttributes::new(
            "Users from a specific journey.".to_string(),
            RumStaticSegmentJourneyQueryObject::new(vec![RumStaticSegmentJourneyNode::new(vec![
                RumStaticSegmentJourneyFilter::new("@type".to_string(), "view".to_string()),
            ])]),
            "My Static Segment".to_string(),
        )
        .tags(vec!["team:frontend".to_string()]),
        RumStaticSegmentRequestType::CREATE_STATIC_SEGMENT_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateRumStaticSegment", true);
    let api = RUMUserSegmentsAPI::with_config(configuration);
    let resp = api.create_rum_static_segment(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
