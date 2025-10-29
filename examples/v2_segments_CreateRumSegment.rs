// Create rum segment returns "Segment created successfully" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_segments::SegmentsAPI;
use datadog_api_client::datadogV2::model::Segment;
use datadog_api_client::datadogV2::model::SegmentData;
use datadog_api_client::datadogV2::model::SegmentDataAttributes;
use datadog_api_client::datadogV2::model::SegmentDataAttributesDataQuery;
use datadog_api_client::datadogV2::model::SegmentDataAttributesDataQueryEventPlatformItems;
use datadog_api_client::datadogV2::model::SegmentDataSource;
use datadog_api_client::datadogV2::model::SegmentDataType;

#[tokio::main]
async fn main() {
    let body = Segment::new().data(
        SegmentData::new(SegmentDataType::SEGMENT)
            .attributes(
                SegmentDataAttributes::new(
                    SegmentDataAttributesDataQuery::new().event_platform(vec![
                        SegmentDataAttributesDataQueryEventPlatformItems::new(
                            "@usr.id".to_string(),
                        )
                        .from("2025-08-01".to_string())
                        .name("high_value_users".to_string())
                        .query(
                            "@type:view @view.name:/logs @usr.session_duration:>300000".to_string(),
                        )
                        .to("2025-09-01".to_string()),
                    ]),
                    "High-Value Users".to_string(),
                )
                .created_at(
                    DateTime::parse_from_rfc3339("0001-01-01T00:00:00+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                )
                .created_by(SegmentDataSource::new(
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ))
                .description(
                    "Users who frequently visit logs and have high session duration".to_string(),
                )
                .modified_at(
                    DateTime::parse_from_rfc3339("0001-01-01T00:00:00+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                )
                .modified_by(SegmentDataSource::new(
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ))
                .org_id(123456)
                .source(0)
                .tags(vec![
                    "high-value".to_string(),
                    "logs".to_string(),
                    "active".to_string(),
                ])
                .version(1),
            )
            .id("segment-12345".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateRumSegment", true);
    let api = SegmentsAPI::with_config(configuration);
    let resp = api.create_rum_segment(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
