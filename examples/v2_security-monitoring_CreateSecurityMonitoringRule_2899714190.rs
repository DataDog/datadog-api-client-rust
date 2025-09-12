// Create a detection rule with detection method 'sequence_detection' returns "OK"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleCaseCreate;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleCreatePayload;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleDetectionMethod;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleEvaluationWindow;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleKeepAlive;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleOptions;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleQueryAggregation;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleSequenceDetectionOptions;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleSequenceDetectionStep;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleSequenceDetectionStepTransition;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleSeverity;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleTypeCreate;
use datadog_api_client::datadogV2::model::SecurityMonitoringStandardDataSource;
use datadog_api_client::datadogV2::model::SecurityMonitoringStandardRuleCreatePayload;
use datadog_api_client::datadogV2::model::SecurityMonitoringStandardRuleQuery;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringRuleCreatePayload::SecurityMonitoringStandardRuleCreatePayload(Box::new(
            SecurityMonitoringStandardRuleCreatePayload::new(
                vec![
                    SecurityMonitoringRuleCaseCreate::new(SecurityMonitoringRuleSeverity::INFO)
                        .condition("step_b > 0".to_string())
                        .name("".to_string())
                        .notifications(vec![]),
                ],
                true,
                "Logs and signals asdf".to_string(),
                "Example-Security-Monitoring".to_string(),
                SecurityMonitoringRuleOptions::new()
                    .detection_method(SecurityMonitoringRuleDetectionMethod::SEQUENCE_DETECTION)
                    .evaluation_window(SecurityMonitoringRuleEvaluationWindow::ZERO_MINUTES)
                    .keep_alive(SecurityMonitoringRuleKeepAlive::FIVE_MINUTES)
                    .max_signal_duration(SecurityMonitoringRuleMaxSignalDuration::TEN_MINUTES)
                    .sequence_detection_options(
                        SecurityMonitoringRuleSequenceDetectionOptions::new()
                            .step_transitions(vec![
                                SecurityMonitoringRuleSequenceDetectionStepTransition::new()
                                    .child("step_b".to_string())
                                    .evaluation_window(
                                        SecurityMonitoringRuleEvaluationWindow::FIFTEEN_MINUTES,
                                    )
                                    .parent("step_a".to_string()),
                            ])
                            .steps(vec![
                                SecurityMonitoringRuleSequenceDetectionStep::new()
                                    .condition("a > 0".to_string())
                                    .evaluation_window(
                                        SecurityMonitoringRuleEvaluationWindow::ONE_MINUTE,
                                    )
                                    .name("step_a".to_string()),
                                SecurityMonitoringRuleSequenceDetectionStep::new()
                                    .condition("b > 0".to_string())
                                    .evaluation_window(
                                        SecurityMonitoringRuleEvaluationWindow::ONE_MINUTE,
                                    )
                                    .name("step_b".to_string()),
                            ]),
                    ),
                vec![
                    SecurityMonitoringStandardRuleQuery::new()
                        .aggregation(SecurityMonitoringRuleQueryAggregation::COUNT)
                        .data_source(SecurityMonitoringStandardDataSource::LOGS)
                        .distinct_fields(vec![])
                        .group_by_fields(vec![])
                        .has_optional_group_by_fields(false)
                        .name("".to_string())
                        .query("service:logs-rule-reducer source:paul test2".to_string()),
                    SecurityMonitoringStandardRuleQuery::new()
                        .aggregation(SecurityMonitoringRuleQueryAggregation::COUNT)
                        .data_source(SecurityMonitoringStandardDataSource::LOGS)
                        .distinct_fields(vec![])
                        .group_by_fields(vec![])
                        .has_optional_group_by_fields(false)
                        .name("".to_string())
                        .query("service:logs-rule-reducer source:paul test1".to_string()),
                ],
            )
            .tags(vec![])
            .type_(SecurityMonitoringRuleTypeCreate::LOG_DETECTION),
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
