// Fetch uptime for multiple tests returns "OK." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsFetchUptimesPayload;

#[tokio::main]
async fn main() {
    let body = SyntheticsFetchUptimesPayload::new(0, vec![], 0);
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.fetch_uptimes(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
