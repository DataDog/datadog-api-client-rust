// Update on-call schedule returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::api_on_call::UpdateOnCallScheduleOptionalParams;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequest;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestData;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataAttributes;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItems;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsInterval;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsMembersItems;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsMembersItemsUser;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsStartDay;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataRelationships;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataRelationshipsTeams;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataRelationshipsTeamsDataItems;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataRelationshipsTeamsDataItemsType;
use datadog_api_client::datadogV2::model::ScheduleUpdateRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "schedule" in the system
    let schedule_data_id = std::env::var("SCHEDULE_DATA_ID").unwrap();
    let schedule_data_relationships_layers_data_0_id =
        std::env::var("SCHEDULE_DATA_RELATIONSHIPS_LAYERS_DATA_0_ID").unwrap();

    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "team" in the system
    let team_data_id = std::env::var("TEAM_DATA_ID").unwrap();
    let body =
        ScheduleUpdateRequest::new(
            ScheduleUpdateRequestData::new(
                ScheduleUpdateRequestDataAttributes::new(
                    vec![
                        ScheduleUpdateRequestDataAttributesLayersItems::new()
                            .effective_date(
                                DateTime::parse_from_rfc3339("2021-11-01T11:11:11+00:00")
                                    .expect("Failed to parse datetime")
                                    .with_timezone(&Utc),
                            )
                            .end_date(
                                DateTime::parse_from_rfc3339("2021-11-21T11:11:11+00:00")
                                    .expect("Failed to parse datetime")
                                    .with_timezone(&Utc),
                            )
                            .id(schedule_data_relationships_layers_data_0_id.clone())
                            .interval(ScheduleUpdateRequestDataAttributesLayersItemsInterval::new().seconds(300))
                            .members(
                                vec![
                                    ScheduleUpdateRequestDataAttributesLayersItemsMembersItems
                                    ::new().user(
                                        ScheduleUpdateRequestDataAttributesLayersItemsMembersItemsUser
                                        ::new().id(user_data_id.clone()),
                                    )
                                ],
                            )
                            .name("Layer 1".to_string())
                            .restrictions(
                                vec![
                                    ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItems::new()
                                        .end_day(
                                            ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay
                                            ::FRIDAY,
                                        )
                                        .end_time("17:00:00".to_string())
                                        .start_day(
                                            ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsStartDay
                                            ::MONDAY,
                                        )
                                        .start_time("09:00:00".to_string())
                                ],
                            )
                            .rotation_start(
                                DateTime::parse_from_rfc3339("2021-11-06T11:11:11+00:00")
                                    .expect("Failed to parse datetime")
                                    .with_timezone(&Utc),
                            )
                    ],
                    "Example-On-Call".to_string(),
                    "America/New_York".to_string(),
                ).tags(vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()]),
                schedule_data_id.clone(),
                ScheduleUpdateRequestDataType::SCHEDULES,
            ).relationships(
                ScheduleUpdateRequestDataRelationships
                ::new().teams(
                    ScheduleUpdateRequestDataRelationshipsTeams
                    ::new().data(
                        vec![
                            ScheduleUpdateRequestDataRelationshipsTeamsDataItems::new()
                                .id(team_data_id.clone())
                                .type_(ScheduleUpdateRequestDataRelationshipsTeamsDataItemsType::TEAMS)
                        ],
                    ),
                ),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .update_on_call_schedule(
            schedule_data_id.clone(),
            body,
            UpdateOnCallScheduleOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
