// Create team notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::TeamNotificationRule;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributes;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributesEmail;
use datadog_api_client::datadogV2::model::TeamNotificationRuleAttributesSlack;
use datadog_api_client::datadogV2::model::TeamNotificationRuleRequest;
use datadog_api_client::datadogV2::model::TeamNotificationRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body = TeamNotificationRuleRequest::new(TeamNotificationRule::new(
        TeamNotificationRuleAttributes::new()
            .email(TeamNotificationRuleAttributesEmail::new().enabled(true))
            .slack(
                TeamNotificationRuleAttributesSlack::new()
                    .channel("aaa-omg-ops".to_string())
                    .workspace("Datadog".to_string()),
            ),
        TeamNotificationRuleType::TEAM_NOTIFICATION_RULES,
    ));
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .create_team_notification_rule(dd_team_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
