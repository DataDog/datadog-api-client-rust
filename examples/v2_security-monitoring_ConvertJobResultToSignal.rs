// Convert a job result to a signal returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::ConvertJobResultsToSignalsAttributes;
use datadog_api_client::datadogV2::model::ConvertJobResultsToSignalsData;
use datadog_api_client::datadogV2::model::ConvertJobResultsToSignalsDataType;
use datadog_api_client::datadogV2::model::ConvertJobResultsToSignalsRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleSeverity;

#[tokio::main]
async fn main() {
    let body = ConvertJobResultsToSignalsRequest::new().data(
        ConvertJobResultsToSignalsData::new()
            .attributes(ConvertJobResultsToSignalsAttributes::new(
                vec!["".to_string()],
                vec!["".to_string()],
                "A large number of failed login attempts.".to_string(),
                SecurityMonitoringRuleSeverity::CRITICAL,
            ))
            .type_(
                ConvertJobResultsToSignalsDataType::HISTORICALDETECTIONSJOBRESULTSIGNALCONVERSION,
            ),
    );

    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ConvertJobResultToSignal", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.convert_job_result_to_signal(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
