// List Bits AI investigations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_ai::BitsAIAPI;
use datadog_api_client::datadogV2::api_bits_ai::ListInvestigationsOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = BitsAIAPI::with_config(configuration);
    let resp = api
        .list_investigations(ListInvestigationsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
