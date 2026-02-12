// Create a detection rule with detection method 'anomaly_detection' with enabled
// feature 'instantaneousBaseline' returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptions;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsBucketDuration;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsDetectionTolerance;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsLearningDuration;
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
use datadog_api_client::datadogV2::model::SecurityMonitoringStandardDataSource;
use datadog_api_client::datadogV2::model::SecurityMonitoringStandardRuleCreatePayload;
use datadog_api_client::datadogV2::model::SecurityMonitoringStandardRuleQuery;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringRuleCreatePayload::SecurityMonitoringStandardRuleCreatePayload(
            Box::new(
                SecurityMonitoringStandardRuleCreatePayload::new(
                    vec![
                        SecurityMonitoringRuleCaseCreate::new(SecurityMonitoringRuleSeverity::INFO)
                            .condition("a > 0.995".to_string())
                            .name("".to_string())
                            .notifications(vec![])
                    ],
                    true,
                    "An anomaly detection rule".to_string(),
                    "Example-Security-Monitoring".to_string(),
                    SecurityMonitoringRuleOptions::new()
                        .anomaly_detection_options(
                            SecurityMonitoringRuleAnomalyDetectionOptions::new()
                                .bucket_duration(
                                    SecurityMonitoringRuleAnomalyDetectionOptionsBucketDuration::FIVE_MINUTES,
                                )
                                .detection_tolerance(
                                    SecurityMonitoringRuleAnomalyDetectionOptionsDetectionTolerance::THREE,
                                )
                                .instantaneous_baseline(true)
                                .learning_duration(
                                    SecurityMonitoringRuleAnomalyDetectionOptionsLearningDuration::ONE_DAY,
                                ),
                        )
                        .detection_method(SecurityMonitoringRuleDetectionMethod::ANOMALY_DETECTION)
                        .evaluation_window(SecurityMonitoringRuleEvaluationWindow::FIFTEEN_MINUTES)
                        .keep_alive(SecurityMonitoringRuleKeepAlive::ONE_HOUR)
                        .max_signal_duration(SecurityMonitoringRuleMaxSignalDuration::ONE_DAY),
                    vec![
                        SecurityMonitoringStandardRuleQuery::new()
                            .aggregation(SecurityMonitoringRuleQueryAggregation::COUNT)
                            .data_source(SecurityMonitoringStandardDataSource::LOGS)
                            .distinct_fields(vec![])
                            .group_by_fields(vec!["@usr.email".to_string(), "@network.client.ip".to_string()])
                            .has_optional_group_by_fields(false)
                            .name("".to_string())
                            .query("service:app status:error".to_string())
                    ],
                )
                    .filters(vec![])
                    .tags(vec![])
                    .type_(SecurityMonitoringRuleTypeCreate::LOG_DETECTION),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_security_monitoring_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
