// Update on-call escalation policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::api_on_call::UpdateOnCallEscalationPolicyOptionalParams;
use datadog_api_client::datadogV2::model::EscalationPolicyStepAttributesAssignment;
use datadog_api_client::datadogV2::model::EscalationPolicyStepTarget;
use datadog_api_client::datadogV2::model::EscalationPolicyStepTargetType;
use datadog_api_client::datadogV2::model::EscalationPolicyUpdateRequest;
use datadog_api_client::datadogV2::model::EscalationPolicyUpdateRequestData;
use datadog_api_client::datadogV2::model::EscalationPolicyUpdateRequestDataAttributes;
use datadog_api_client::datadogV2::model::EscalationPolicyUpdateRequestDataAttributesStepsItems;
use datadog_api_client::datadogV2::model::EscalationPolicyUpdateRequestDataRelationships;
use datadog_api_client::datadogV2::model::EscalationPolicyUpdateRequestDataRelationshipsTeams;
use datadog_api_client::datadogV2::model::EscalationPolicyUpdateRequestDataRelationshipsTeamsDataItems;
use datadog_api_client::datadogV2::model::EscalationPolicyUpdateRequestDataRelationshipsTeamsDataItemsType;
use datadog_api_client::datadogV2::model::EscalationPolicyUpdateRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "escalation_policy" in the system
    let escalation_policy_data_id = std::env::var("ESCALATION_POLICY_DATA_ID").unwrap();
    let escalation_policy_data_relationships_steps_data_0_id =
        std::env::var("ESCALATION_POLICY_DATA_RELATIONSHIPS_STEPS_DATA_0_ID").unwrap();

    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body = EscalationPolicyUpdateRequest::new(
        EscalationPolicyUpdateRequestData::new(
            EscalationPolicyUpdateRequestDataAttributes::new(
                "Example-On-Call-updated".to_string(),
                vec![
                    EscalationPolicyUpdateRequestDataAttributesStepsItems::new(vec![
                        EscalationPolicyStepTarget::new()
                            .id(user_data_id.clone())
                            .type_(EscalationPolicyStepTargetType::USERS),
                    ])
                    .assignment(EscalationPolicyStepAttributesAssignment::DEFAULT)
                    .escalate_after_seconds(3600)
                    .id(escalation_policy_data_relationships_steps_data_0_id.clone()),
                ],
            )
            .description("Example-On-Call".to_string())
            .resolve_page_on_policy_end(false)
            .retries(0),
            escalation_policy_data_id.clone(),
            EscalationPolicyUpdateRequestDataType::POLICIES,
        )
        .relationships(EscalationPolicyUpdateRequestDataRelationships::new().teams(
            EscalationPolicyUpdateRequestDataRelationshipsTeams::new().data(vec![
                EscalationPolicyUpdateRequestDataRelationshipsTeamsDataItems::new(
                    dd_team_data_id.clone(),
                    EscalationPolicyUpdateRequestDataRelationshipsTeamsDataItemsType::TEAMS,
                ),
            ]),
        )),
    );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .update_on_call_escalation_policy(
            escalation_policy_data_id.clone(),
            body,
            UpdateOnCallEscalationPolicyOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
