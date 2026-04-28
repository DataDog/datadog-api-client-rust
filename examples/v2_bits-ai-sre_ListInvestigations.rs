// List Bits AI SRE investigations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_aisre::BitsAISREAPI;
use datadog_api_client::datadogV2::api_bits_aisre::ListInvestigationsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListInvestigations", true);
    let api = BitsAISREAPI::with_config(configuration);
    let resp = api
        .list_investigations(ListInvestigationsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
