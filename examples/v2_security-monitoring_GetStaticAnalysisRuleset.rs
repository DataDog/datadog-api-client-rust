// Get a SAST ruleset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::GetStaticAnalysisRulesetOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetStaticAnalysisRuleset", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .get_static_analysis_ruleset(
            "python-best-practices".to_string(),
            GetStaticAnalysisRulesetOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
