// List unified hosts returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_settings::CSMSettingsAPI;
use datadog_api_client::datadogV2::api_csm_settings::ListCSMUnifiedHostsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListCSMUnifiedHosts", true);
    let api = CSMSettingsAPI::with_config(configuration);
    let resp = api
        .list_csm_unified_hosts(ListCSMUnifiedHostsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
