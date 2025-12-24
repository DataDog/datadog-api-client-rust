// Update team notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::TeamNotificationRule;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributes;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributesPagerduty;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributesSlack;
use datadog_api_client::datadogV2::model::TeamNotificationRuleRequest;
use datadog_api_client::datadogV2::model::TeamNotificationRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();

    // there is a valid "team_notification_rule" in the system
    let team_notification_rule_data_id = std::env::var("TEAM_NOTIFICATION_RULE_DATA_ID").unwrap();
    let body = TeamNotificationRuleRequest::new(
        TeamNotificationRule::new(
            TeamNotificationRuleAttributes::new()
                .pagerduty(
                    TeamNotificationRuleAttributesPagerduty::new()
                        .service_name("Datadog-prod".to_string()),
                )
                .slack(
                    TeamNotificationRuleAttributesSlack::new()
                        .channel("aaa-governance-ops".to_string())
                        .workspace("Datadog".to_string()),
                ),
            TeamNotificationRuleType::TEAM_NOTIFICATION_RULES,
        )
        .id(team_notification_rule_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .update_team_notification_rule(
            dd_team_data_id.clone(),
            team_notification_rule_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
