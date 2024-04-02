// Get an API test's latest results summaries returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api::api_synthetics::GetAPITestLatestResultsOptionalParams;
use datadog_api_client::datadogV1::api::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .get_api_test_latest_results(
            "hwb-332-3xe".to_string(),
            GetAPITestLatestResultsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
