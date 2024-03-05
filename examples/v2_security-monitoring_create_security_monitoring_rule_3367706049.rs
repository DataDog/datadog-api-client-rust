// Create a detection rule with detection method 'third_party' returns "OK" response
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
                    vec![],
                    true,
                    "This is a third party rule".to_string(),
                    "Example-Security-Monitoring".to_string(),
                    SecurityMonitoringRuleOptions::new()
                        .detection_method(SecurityMonitoringRuleDetectionMethod::THIRD_PARTY)
                        .keep_alive(SecurityMonitoringRuleKeepAlive::ZERO_MINUTES)
                        .max_signal_duration(SecurityMonitoringRuleMaxSignalDuration::ZERO_MINUTES)
                        .third_party_rule_options(
                            SecurityMonitoringRuleThirdPartyOptions::new()
                                .default_status(SecurityMonitoringRuleSeverity::INFO)
                                .root_queries(
                                    vec![
                                        SecurityMonitoringThirdPartyRootQuery::new()
                                            .group_by_fields(vec!["instance-id".to_string()])
                                            .query("source:guardduty @details.alertType:*EC2*".to_string()),
                                        SecurityMonitoringThirdPartyRootQuery::new()
                                            .group_by_fields(vec![])
                                            .query("source:guardduty".to_string())
                                    ],
                                ),
                        ),
                    vec![],
                )
                    .third_party_cases(
                        vec![
                            SecurityMonitoringThirdPartyRuleCaseCreate::new(SecurityMonitoringRuleSeverity::HIGH)
                                .name("high".to_string())
                                .query("status:error".to_string()),
                            SecurityMonitoringThirdPartyRuleCaseCreate::new(SecurityMonitoringRuleSeverity::LOW)
                                .name("low".to_string())
                                .query("status:info".to_string())
                        ],
                    )
                    .type_(SecurityMonitoringRuleTypeCreate::LOG_DETECTION),
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
