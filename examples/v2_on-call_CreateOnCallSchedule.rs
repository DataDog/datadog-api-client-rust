// Create On-Call schedule returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::CreateOnCallScheduleOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::DataRelationshipsTeams;
use datadog_api_client::datadogV2::model::DataRelationshipsTeamsDataItems;
use datadog_api_client::datadogV2::model::DataRelationshipsTeamsDataItemsType;
use datadog_api_client::datadogV2::model::LayerAttributesInterval;
use datadog_api_client::datadogV2::model::ScheduleCreateRequest;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestData;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributes;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataAttributesLayersItems;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataRelationships;
use datadog_api_client::datadogV2::model::ScheduleCreateRequestDataType;
use datadog_api_client::datadogV2::model::ScheduleRequestDataAttributesLayersItemsMembersItems;
use datadog_api_client::datadogV2::model::ScheduleRequestDataAttributesLayersItemsMembersItemsUser;
use datadog_api_client::datadogV2::model::TimeRestriction;
use datadog_api_client::datadogV2::model::Weekday;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body = ScheduleCreateRequest::new(
        ScheduleCreateRequestData::new(
            ScheduleCreateRequestDataAttributes::new(
                vec![ScheduleCreateRequestDataAttributesLayersItems::new(
                    DateTime::parse_from_rfc3339("2021-11-01T11:11:11+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                    LayerAttributesInterval::new().days(1),
                    vec![
                        ScheduleRequestDataAttributesLayersItemsMembersItems::new().user(
                            ScheduleRequestDataAttributesLayersItemsMembersItemsUser::new()
                                .id(user_data_id.clone()),
                        ),
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
                .restrictions(vec![TimeRestriction::new()
                    .end_day(Weekday::FRIDAY)
                    .end_time("17:00:00".to_string())
                    .start_day(Weekday::MONDAY)
                    .start_time("09:00:00".to_string())])],
                "Example-On-Call".to_string(),
                "America/New_York".to_string(),
            ),
            ScheduleCreateRequestDataType::SCHEDULES,
        )
        .relationships(ScheduleCreateRequestDataRelationships::new().teams(
            DataRelationshipsTeams::new().data(vec![DataRelationshipsTeamsDataItems::new(
                dd_team_data_id.clone(),
                DataRelationshipsTeamsDataItemsType::TEAMS,
            )]),
        )),
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
