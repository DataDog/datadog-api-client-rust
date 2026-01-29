// Update a notification rule returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseNotificationRuleAttributes;
use datadog_api_client::datadogV2::model::CaseNotificationRuleRecipient;
use datadog_api_client::datadogV2::model::CaseNotificationRuleRecipientData;
use datadog_api_client::datadogV2::model::CaseNotificationRuleResourceType;
use datadog_api_client::datadogV2::model::CaseNotificationRuleTrigger;
use datadog_api_client::datadogV2::model::CaseNotificationRuleTriggerData;
use datadog_api_client::datadogV2::model::CaseNotificationRuleUpdate;
use datadog_api_client::datadogV2::model::CaseNotificationRuleUpdateRequest;

#[tokio::main]
async fn main() {
    let body = CaseNotificationRuleUpdateRequest::new(
        CaseNotificationRuleUpdate::new(CaseNotificationRuleResourceType::NOTIFICATION_RULE)
            .attributes(
                CaseNotificationRuleAttributes::new()
                    .recipients(vec![CaseNotificationRuleRecipient::new()
                        .data(CaseNotificationRuleRecipientData::new())
                        .type_("EMAIL".to_string())])
                    .triggers(vec![CaseNotificationRuleTrigger::new()
                        .data(CaseNotificationRuleTriggerData::new())
                        .type_("CASE_CREATED".to_string())]),
            ),
    );
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .update_project_notification_rule(
            "project_id".to_string(),
            "notification_rule_id".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
