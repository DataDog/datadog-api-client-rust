// Create a detection rule with type 'application_security 'returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleCaseAction;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleCaseActionOptions;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleCaseActionType;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleCaseCreate;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleCreatePayload;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleDetectionMethod;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleEvaluationWindow;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleKeepAlive;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleOptions;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleQueryAggregation;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleSeverity;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleTypeCreate;
use datadog_api_client::datadogV2::model::SecurityMonitoringStandardRuleCreatePayload;
use datadog_api_client::datadogV2::model::SecurityMonitoringStandardRuleQuery;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringRuleCreatePayload::SecurityMonitoringStandardRuleCreatePayload(Box::new(
            SecurityMonitoringStandardRuleCreatePayload::new(
                vec![
                    SecurityMonitoringRuleCaseCreate::new(SecurityMonitoringRuleSeverity::INFO)
                        .actions(vec![
                            SecurityMonitoringRuleCaseAction::new()
                                .options(
                                    SecurityMonitoringRuleCaseActionOptions::new().duration(900),
                                )
                                .type_(SecurityMonitoringRuleCaseActionType::BLOCK_IP),
                            SecurityMonitoringRuleCaseAction::new()
                                .options(
                                    SecurityMonitoringRuleCaseActionOptions::new()
                                        .user_behavior_name("behavior".to_string()),
                                )
                                .type_(SecurityMonitoringRuleCaseActionType::USER_BEHAVIOR),
                        ])
                        .condition("a > 100000".to_string())
                        .name("".to_string())
                        .notifications(vec![]),
                ],
                true,
                "Test rule".to_string(),
                "Example-Security-Monitoring_appsec_rule".to_string(),
                SecurityMonitoringRuleOptions::new()
                    .detection_method(SecurityMonitoringRuleDetectionMethod::THRESHOLD)
                    .evaluation_window(SecurityMonitoringRuleEvaluationWindow::FIFTEEN_MINUTES)
                    .keep_alive(SecurityMonitoringRuleKeepAlive::ONE_HOUR)
                    .max_signal_duration(SecurityMonitoringRuleMaxSignalDuration::ONE_DAY),
                vec![SecurityMonitoringStandardRuleQuery::new()
                    .aggregation(SecurityMonitoringRuleQueryAggregation::COUNT)
                    .distinct_fields(vec![])
                    .group_by_fields(vec!["service".to_string(), "@http.client_ip".to_string()])
                    .query(
                        "@appsec.security_activity:business_logic.users.login.failure".to_string(),
                    )],
            )
            .filters(vec![])
            .group_signals_by(vec!["service".to_string()])
            .tags(vec![])
            .type_(SecurityMonitoringRuleTypeCreate::APPLICATION_SECURITY),
        ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_security_monitoring_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
