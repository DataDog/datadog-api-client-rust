// Fetch uptime for multiple tests returns "OK." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsFetchUptimesPayload;

#[tokio::main]
async fn main() {
    let body =
        SyntheticsFetchUptimesPayload::new(1726041488, vec!["p8m-9gw-nte".to_string()], 1726055954);
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.fetch_uptimes(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
