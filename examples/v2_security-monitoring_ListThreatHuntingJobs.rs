// List threat hunting jobs returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListThreatHuntingJobsOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListThreatHuntingJobs", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_threat_hunting_jobs(ListThreatHuntingJobsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
