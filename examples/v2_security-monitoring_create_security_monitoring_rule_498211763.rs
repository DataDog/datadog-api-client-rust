// Create a detection rule with type 'workload_security' returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringRuleCreatePayload::SecurityMonitoringStandardRuleCreatePayload(
            Box::new(
                SecurityMonitoringStandardRuleCreatePayload::new(
                    vec![
                        SecurityMonitoringRuleCaseCreate::new(SecurityMonitoringRuleSeverity::INFO)
                            .condition("a > 0".to_string())
                            .name("".to_string())
                            .notifications(vec![])
                    ],
                    true,
                    "Test rule".to_string(),
                    "Example-Security-Monitoring".to_string(),
                    SecurityMonitoringRuleOptions::new()
                        .evaluation_window(SecurityMonitoringRuleEvaluationWindow::FIFTEEN_MINUTES)
                        .keep_alive(SecurityMonitoringRuleKeepAlive::ONE_HOUR)
                        .max_signal_duration(SecurityMonitoringRuleMaxSignalDuration::ONE_DAY),
                    vec![
                        SecurityMonitoringStandardRuleQuery::new()
                            .aggregation(SecurityMonitoringRuleQueryAggregation::COUNT)
                            .distinct_fields(vec![])
                            .group_by_fields(vec![])
                            .metric("".to_string())
                            .query("@test:true".to_string())
                    ],
                )
                    .filters(vec![])
                    .tags(vec![])
                    .type_(SecurityMonitoringRuleTypeCreate::WORKLOAD_SECURITY),
            ),
        );
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_security_monitoring_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
