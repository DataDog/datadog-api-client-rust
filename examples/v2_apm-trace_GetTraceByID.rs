// Get a trace by ID returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_apm_trace::APMTraceAPI;
use datadog_api_client::datadogV2::api_apm_trace::GetTraceByIDOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetTraceByID", true);
    let api = APMTraceAPI::with_config(configuration);
    let resp = api
        .get_trace_by_id(
            "trace_id".to_string(),
            GetTraceByIDOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
