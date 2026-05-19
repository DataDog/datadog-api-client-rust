// Update an automation rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::AutomationRuleAction;
use datadog_api_client::datadogV2::model::AutomationRuleActionData;
use datadog_api_client::datadogV2::model::AutomationRuleActionType;
use datadog_api_client::datadogV2::model::AutomationRuleCreateAttributes;
use datadog_api_client::datadogV2::model::AutomationRuleTrigger;
use datadog_api_client::datadogV2::model::AutomationRuleTriggerData;
use datadog_api_client::datadogV2::model::AutomationRuleTriggerType;
use datadog_api_client::datadogV2::model::AutomationRuleUpdate;
use datadog_api_client::datadogV2::model::AutomationRuleUpdateRequest;
use datadog_api_client::datadogV2::model::CaseAutomationRuleResourceType;
use datadog_api_client::datadogV2::model::CaseAutomationRuleState;

#[tokio::main]
async fn main() {
    let body = AutomationRuleUpdateRequest::new(
        AutomationRuleUpdate::new(CaseAutomationRuleResourceType::RULE).attributes(
            AutomationRuleCreateAttributes::new(
                AutomationRuleAction::new(
                    AutomationRuleActionData::new().handle("workflow-handle-123".to_string()),
                    AutomationRuleActionType::EXECUTE_WORKFLOW,
                ),
                "Auto-assign workflow".to_string(),
                AutomationRuleTrigger::new(AutomationRuleTriggerType::CASE_CREATED)
                    .data(AutomationRuleTriggerData::new()),
            )
            .state(CaseAutomationRuleState::ENABLED),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateCaseAutomationRule", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .update_case_automation_rule("project_id".to_string(), "rule_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
