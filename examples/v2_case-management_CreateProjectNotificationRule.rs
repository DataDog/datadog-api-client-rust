// Create a notification rule returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseNotificationRuleCreate;
use datadog_api_client::datadogV2::model::CaseNotificationRuleCreateAttributes;
use datadog_api_client::datadogV2::model::CaseNotificationRuleCreateRequest;
use datadog_api_client::datadogV2::model::CaseNotificationRuleRecipient;
use datadog_api_client::datadogV2::model::CaseNotificationRuleRecipientData;
use datadog_api_client::datadogV2::model::CaseNotificationRuleResourceType;
use datadog_api_client::datadogV2::model::CaseNotificationRuleTrigger;
use datadog_api_client::datadogV2::model::CaseNotificationRuleTriggerData;

#[tokio::main]
async fn main() {
    let body = CaseNotificationRuleCreateRequest::new(CaseNotificationRuleCreate::new(
        CaseNotificationRuleCreateAttributes::new(
            vec![CaseNotificationRuleRecipient::new()
                .data(CaseNotificationRuleRecipientData::new())
                .type_("EMAIL".to_string())],
            vec![CaseNotificationRuleTrigger::new()
                .data(CaseNotificationRuleTriggerData::new())
                .type_("CASE_CREATED".to_string())],
        )
        .is_enabled(true),
        CaseNotificationRuleResourceType::NOTIFICATION_RULE,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .create_project_notification_rule("project_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
