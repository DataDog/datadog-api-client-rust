// Run a threat hunting job returns "Status created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::JobDefinition;
use datadog_api_client::datadogV2::model::RunThreatHuntingJobRequest;
use datadog_api_client::datadogV2::model::RunThreatHuntingJobRequestAttributes;
use datadog_api_client::datadogV2::model::RunThreatHuntingJobRequestData;
use datadog_api_client::datadogV2::model::RunThreatHuntingJobRequestDataType;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleCaseCreate;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleEvaluationWindow;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleKeepAlive;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleQueryAggregation;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleSeverity;
use datadog_api_client::datadogV2::model::ThreatHuntingJobOptions;
use datadog_api_client::datadogV2::model::ThreatHuntingJobQuery;

#[tokio::main]
async fn main() {
    let body = RunThreatHuntingJobRequest::new().data(
        RunThreatHuntingJobRequestData::new()
            .attributes(
                RunThreatHuntingJobRequestAttributes::new().job_definition(
                    JobDefinition::new(
                        vec![SecurityMonitoringRuleCaseCreate::new(
                            SecurityMonitoringRuleSeverity::INFO,
                        )
                        .condition("a > 1".to_string())
                        .name("Condition 1".to_string())
                        .notifications(vec![])],
                        1730387522611,
                        "main".to_string(),
                        "A large number of failed login attempts.".to_string(),
                        "Excessive number of failed attempts.".to_string(),
                        vec![ThreatHuntingJobQuery::new()
                            .aggregation(SecurityMonitoringRuleQueryAggregation::COUNT)
                            .distinct_fields(vec![])
                            .group_by_fields(vec![])
                            .query("source:non_existing_src_weekend".to_string())],
                        1730387532611,
                    )
                    .options(
                        ThreatHuntingJobOptions::new()
                            .evaluation_window(
                                SecurityMonitoringRuleEvaluationWindow::FIFTEEN_MINUTES,
                            )
                            .keep_alive(SecurityMonitoringRuleKeepAlive::ONE_HOUR)
                            .max_signal_duration(SecurityMonitoringRuleMaxSignalDuration::ONE_DAY),
                    )
                    .tags(vec![])
                    .type_("log_detection".to_string()),
                ),
            )
            .type_(RunThreatHuntingJobRequestDataType::HISTORICALDETECTIONSJOBCREATE),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.RunThreatHuntingJob", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.run_threat_hunting_job(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
