// List Bits AI SRE investigations returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_aisre::BitsAISREAPI;
use datadog_api_client::datadogV2::api_bits_aisre::ListInvestigationsOptionalParams;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListInvestigations", true);
    let api = BitsAISREAPI::with_config(configuration);
    let response =
        api.list_investigations_with_pagination(ListInvestigationsOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
