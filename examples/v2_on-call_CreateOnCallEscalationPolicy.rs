// Create on-call escalation policy returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::CreateOnCallEscalationPolicyOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequest;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestData;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataAttributes;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItems;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsAssignment;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItems;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItemsType;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataRelationships;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataRelationshipsTeams;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataRelationshipsTeamsDataItems;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataRelationshipsTeamsDataItemsType;
use datadog_api_client::datadogV2::model::EscalationPolicyCreateRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "schedule" in the system
    let schedule_data_id = std::env::var("SCHEDULE_DATA_ID").unwrap();

    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body =
        EscalationPolicyCreateRequest::new(
            EscalationPolicyCreateRequestData::new(
                EscalationPolicyCreateRequestDataAttributes::new(
                    "Example-On-Call".to_string(),
                    vec![
                        EscalationPolicyCreateRequestDataAttributesStepsItems::new(
                            vec![
                                EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItems::new()
                                    .id(user_data_id.clone())
                                    .type_(
                                        EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItemsType::USERS,
                                    ),
                                EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItems::new()
                                    .id(schedule_data_id.clone())
                                    .type_(
                                        EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItemsType
                                        ::SCHEDULES,
                                    ),
                                EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItems::new()
                                    .id(dd_team_data_id.clone())
                                    .type_(
                                        EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItemsType::TEAMS,
                                    )
                            ],
                        )
                            .assignment(EscalationPolicyCreateRequestDataAttributesStepsItemsAssignment::DEFAULT)
                            .escalate_after_seconds(3600)
                    ],
                )
                    .description("Escalation Policy 1 description".to_string())
                    .resolve_page_on_policy_end(true)
                    .retries(2),
                EscalationPolicyCreateRequestDataType::POLICIES,
            ).relationships(
                EscalationPolicyCreateRequestDataRelationships
                ::new().teams(
                    EscalationPolicyCreateRequestDataRelationshipsTeams
                    ::new().data(
                        vec![
                            EscalationPolicyCreateRequestDataRelationshipsTeamsDataItems::new(
                                dd_team_data_id.clone(),
                                EscalationPolicyCreateRequestDataRelationshipsTeamsDataItemsType::TEAMS,
                            )
                        ],
                    ),
                ),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .create_on_call_escalation_policy(
            body,
            CreateOnCallEscalationPolicyOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
