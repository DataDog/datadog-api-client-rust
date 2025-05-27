// Set On-Call team routing rules returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::api_on_call::SetOnCallTeamRoutingRulesOptionalParams;
use datadog_api_client::datadogV2::model::RoutingRuleAction;
use datadog_api_client::datadogV2::model::SendSlackMessageAction;
use datadog_api_client::datadogV2::model::SendSlackMessageActionType;
use datadog_api_client::datadogV2::model::TeamRoutingRulesRequest;
use datadog_api_client::datadogV2::model::TeamRoutingRulesRequestData;
use datadog_api_client::datadogV2::model::TeamRoutingRulesRequestDataAttributes;
use datadog_api_client::datadogV2::model::TeamRoutingRulesRequestDataType;
use datadog_api_client::datadogV2::model::TeamRoutingRulesRequestRule;
use datadog_api_client::datadogV2::model::TimeRestriction;
use datadog_api_client::datadogV2::model::TimeRestrictions;
use datadog_api_client::datadogV2::model::Urgency;
use datadog_api_client::datadogV2::model::Weekday;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();

    // there is a valid "escalation_policy" in the system
    let escalation_policy_data_id = std::env::var("ESCALATION_POLICY_DATA_ID").unwrap();
    let body = TeamRoutingRulesRequest::new().data(
        TeamRoutingRulesRequestData::new(TeamRoutingRulesRequestDataType::TEAM_ROUTING_RULES)
            .attributes(TeamRoutingRulesRequestDataAttributes::new().rules(vec![
                            TeamRoutingRulesRequestRule::new()
                                .actions(
                                    vec![
                                        RoutingRuleAction::SendSlackMessageAction(
                                            Box::new(
                                                SendSlackMessageAction::new(
                                                    "channel".to_string(),
                                                    SendSlackMessageActionType::SEND_SLACK_MESSAGE,
                                                    "workspace".to_string(),
                                                ),
                                            ),
                                        )
                                    ],
                                )
                                .query("tags.service:test".to_string())
                                .time_restriction(
                                    TimeRestrictions::new(
                                        vec![
                                            TimeRestriction::new()
                                                .end_day(Weekday::MONDAY)
                                                .end_time("17:00:00".to_string())
                                                .start_day(Weekday::MONDAY)
                                                .start_time("09:00:00".to_string()),
                                            TimeRestriction::new()
                                                .end_day(Weekday::TUESDAY)
                                                .end_time("17:00:00".to_string())
                                                .start_day(Weekday::TUESDAY)
                                                .start_time("09:00:00".to_string())
                                        ],
                                        "Europe/Paris".to_string(),
                                    ),
                                ),
                            TeamRoutingRulesRequestRule::new()
                                .policy_id(escalation_policy_data_id.clone())
                                .query("".to_string())
                                .urgency(Urgency::LOW)
                        ]))
            .id(dd_team_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .set_on_call_team_routing_rules(
            dd_team_data_id.clone(),
            body,
            SetOnCallTeamRoutingRulesOptionalParams::default().include("rules".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
