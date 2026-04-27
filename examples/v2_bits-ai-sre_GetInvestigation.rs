// Get a Bits AI SRE investigation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_aisre::BitsAISREAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetInvestigation", true);
    let api = BitsAISREAPI::with_config(configuration);
    let resp = api.get_investigation("id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
