// Create on-call schedule returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::CreateOnCallScheduleOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::ScheduleCreateRequest;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestData;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributes;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItems;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItemsInterval;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItemsMembersItems;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItemsMembersItemsUser;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItemsRestrictionsItems;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItemsRestrictionsItemsEndDay;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItemsRestrictionsItemsStartDay;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataRelationships;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataRelationshipsTeams;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataRelationshipsTeamsDataItems;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataRelationshipsTeamsDataItemsType;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body =
        ScheduleCreateRequest::new(
            ScheduleCreateRequestData::new(ScheduleCreateRequestDataType::SCHEDULES)
                .attributes(
                    ScheduleCreateRequestDataAttributes::new(
                        vec![
                            ScheduleCreateRequestDataAttributesLayersItems::new(
                                DateTime::parse_from_rfc3339("2021-11-01T11:11:11+00:00")
                                    .expect("Failed to parse datetime")
                                    .with_timezone(&Utc),
                                ScheduleCreateRequestDataAttributesLayersItemsInterval::new().days(1),
                                vec![
                                    ScheduleCreateRequestDataAttributesLayersItemsMembersItems
                                    ::new().user(
                                        ScheduleCreateRequestDataAttributesLayersItemsMembersItemsUser
                                        ::new().id(user_data_id.clone()),
                                    )
                                ],
                                "Layer 1".to_string(),
                                DateTime::parse_from_rfc3339("2021-11-06T11:11:11+00:00")
                                    .expect("Failed to parse datetime")
                                    .with_timezone(&Utc),
                            )
                                .end_date(
                                    DateTime::parse_from_rfc3339("2021-11-21T11:11:11+00:00")
                                        .expect("Failed to parse datetime")
                                        .with_timezone(&Utc),
                                )
                                .restrictions(
                                    vec![
                                        ScheduleCreateRequestDataAttributesLayersItemsRestrictionsItems::new()
                                            .end_day(
                                                ScheduleCreateRequestDataAttributesLayersItemsRestrictionsItemsEndDay
                                                ::FRIDAY,
                                            )
                                            .end_time("17:00:00".to_string())
                                            .start_day(
                                                ScheduleCreateRequestDataAttributesLayersItemsRestrictionsItemsStartDay
                                                ::MONDAY,
                                            )
                                            .start_time("09:00:00".to_string())
                                    ],
                                )
                        ],
                        "Example-On-Call".to_string(),
                        "America/New_York".to_string(),
                    ).tags(vec!["tag1".to_string(), "tag2".to_string()]),
                )
                .relationships(
                    ScheduleCreateRequestDataRelationships
                    ::new().teams(
                        ScheduleCreateRequestDataRelationshipsTeams
                        ::new().data(
                            vec![
                                ScheduleCreateRequestDataRelationshipsTeamsDataItems::new(
                                    dd_team_data_id.clone(),
                                    ScheduleCreateRequestDataRelationshipsTeamsDataItemsType::TEAMS,
                                )
                            ],
                        ),
                    ),
                ),
        );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .create_on_call_schedule(body, CreateOnCallScheduleOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
