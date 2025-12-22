// Create team notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::TeamNotificationRule;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributes;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributesEmail;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributesMsTeams;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributesPagerduty;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributesSlack;

#[tokio::main]
async fn main() {
    let body = TeamNotificationRule::new()
        .attributes(
            TeamNotificationRuleAttributes::new()
                .email(TeamNotificationRuleAttributesEmail::new())
                .ms_teams(TeamNotificationRuleAttributesMsTeams::new())
                .pagerduty(TeamNotificationRuleAttributesPagerduty::new())
                .slack(TeamNotificationRuleAttributesSlack::new()),
        )
        .id("b8626d7e-cedd-11eb-abf5-da7ad0900001".to_string());
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .create_team_notification_rule("team_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
