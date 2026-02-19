// Update a RUM segment returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_user_segments::RUMUserSegmentsAPI;
use datadog_api_client::datadogV2::model::RumSegmentDataQuery;
use datadog_api_client::datadogV2::model::RumSegmentEventPlatform;
use datadog_api_client::datadogV2::model::RumSegmentJourney;
use datadog_api_client::datadogV2::model::RumSegmentReferenceTable;
use datadog_api_client::datadogV2::model::RumSegmentReferenceTableColumn;
use datadog_api_client::datadogV2::model::RumSegmentReferenceTableJoinCondition;
use datadog_api_client::datadogV2::model::RumSegmentResourceType;
use datadog_api_client::datadogV2::model::RumSegmentStaticEntry;
use datadog_api_client::datadogV2::model::RumSegmentTemplateInstance;
use datadog_api_client::datadogV2::model::RumSegmentUpdateAttributes;
use datadog_api_client::datadogV2::model::RumSegmentUpdateData;
use datadog_api_client::datadogV2::model::RumSegmentUpdateRequest;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = RumSegmentUpdateRequest::new(RumSegmentUpdateData::new(
        RumSegmentUpdateAttributes::new()
            .data_query(
                RumSegmentDataQuery::new()
                    .combination("(logs && apm_home) && NOT(apm_trace)".to_string())
                    .event_platforms(vec![RumSegmentEventPlatform::new(
                        "@usr.id".to_string(),
                        "logs".to_string(),
                        "@type:view @view.url_path:/logs".to_string(),
                    )
                    .from(1709888355000)
                    .to(1710493155000)])
                    .journeys(vec![RumSegmentJourney::new()
                        .conversion_type("any".to_string())
                        .group_by("@usr.id".to_string())
                        .name("my_journey".to_string())
                        .search("@type:view".to_string())])
                    .reference_tables(vec![RumSegmentReferenceTable::new(
                        vec![RumSegmentReferenceTableColumn::new("user_id".to_string())],
                        RumSegmentReferenceTableJoinCondition::new(
                            "user_id".to_string(),
                            "@usr.id".to_string(),
                        ),
                        "my_ref_table".to_string(),
                        "my_table".to_string(),
                    )
                    .filter_query("".to_string())])
                    .static_(vec![RumSegmentStaticEntry::new(
                        "static-list-1".to_string(),
                        "My Static List".to_string(),
                        500,
                    )])
                    .templates(vec![RumSegmentTemplateInstance::new(
                        "stickiness-v1".to_string(),
                    )
                    .from(1709888355000)
                    .parameters(BTreeMap::from([("threshold".to_string(), "5".to_string())]))
                    .to(1710493155000)]),
            )
            .description("Updated description.".to_string())
            .name("Updated Segment Name".to_string())
            .tags(vec!["team:backend".to_string()]),
        "a1b2c3d4-1234-5678-9abc-123456789abc".to_string(),
        RumSegmentResourceType::SEGMENT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateRumSegment", true);
    let api = RUMUserSegmentsAPI::with_config(configuration);
    let resp = api
        .update_rum_segment("a1b2c3d4-1234-5678-9abc-123456789abc".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
