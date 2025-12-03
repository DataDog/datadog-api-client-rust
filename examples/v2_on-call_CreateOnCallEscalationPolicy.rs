// Create On-Call escalation policy returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::CreateOnCallEscalationPolicyOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::DataRelationshipsTeams;
use datadog_api_client::datadogV2::model::DataRelationshipsTeamsDataItems;
use datadog_api_client::datadogV2::model::DataRelationshipsTeamsDataItemsType;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequest;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestData;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataAttributes;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItems;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataRelationships;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataType;
use datadog_api_client::datadogV2::model::EscalationPolicyStepAttributesAssignment;
use datadog_api_client::datadogV2::model::EscalationPolicyStepTarget;
use datadog_api_client::datadogV2::model::EscalationPolicyStepTargetConfig;
use datadog_api_client::datadogV2::model::EscalationPolicyStepTargetConfigSchedule;
use datadog_api_client::datadogV2::model::EscalationPolicyStepTargetType;
use datadog_api_client::datadogV2::model::ScheduleTargetPosition;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "schedule" in the system
    let schedule_data_id = std::env::var("SCHEDULE_DATA_ID").unwrap();

    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body = EscalationPolicyCreateRequest::new(
        EscalationPolicyCreateRequestData::new(
            EscalationPolicyCreateRequestDataAttributes::new(
                "Example-On-Call".to_string(),
                vec![
                    EscalationPolicyCreateRequestDataAttributesStepsItems::new(vec![
                        EscalationPolicyStepTarget::new()
                            .id(user_data_id.clone())
                            .type_(EscalationPolicyStepTargetType::USERS),
                        EscalationPolicyStepTarget::new()
                            .id(schedule_data_id.clone())
                            .type_(EscalationPolicyStepTargetType::SCHEDULES),
                        EscalationPolicyStepTarget::new()
                            .config(
                                EscalationPolicyStepTargetConfig::new().schedule(
                                    EscalationPolicyStepTargetConfigSchedule::new()
                                        .position(ScheduleTargetPosition::PREVIOUS),
                                ),
                            )
                            .id(schedule_data_id.clone())
                            .type_(EscalationPolicyStepTargetType::SCHEDULES),
                        EscalationPolicyStepTarget::new()
                            .id(dd_team_data_id.clone())
                            .type_(EscalationPolicyStepTargetType::TEAMS),
                    ])
                    .assignment(EscalationPolicyStepAttributesAssignment::DEFAULT)
                    .escalate_after_seconds(3600),
                    EscalationPolicyCreateRequestDataAttributesStepsItems::new(vec![
                        EscalationPolicyStepTarget::new()
                            .id(dd_team_data_id.clone())
                            .type_(EscalationPolicyStepTargetType::TEAMS),
                    ])
                    .assignment(EscalationPolicyStepAttributesAssignment::ROUND_ROBIN)
                    .escalate_after_seconds(3600),
                ],
            )
            .resolve_page_on_policy_end(true)
            .retries(2),
            EscalationPolicyCreateRequestDataType::POLICIES,
        )
        .relationships(EscalationPolicyCreateRequestDataRelationships::new().teams(
            DataRelationshipsTeams::new().data(vec![DataRelationshipsTeamsDataItems::new(
                dd_team_data_id.clone(),
                DataRelationshipsTeamsDataItemsType::TEAMS,
            )]),
        )),
    );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .create_on_call_escalation_policy(
            body,
            CreateOnCallEscalationPolicyOptionalParams::default()
                .include("steps.targets".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
