// Get agentless host facet info returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_settings::CSMSettingsAPI;
use datadog_api_client::datadogV2::api_csm_settings::GetCSMAgentlessHostFacetInfoOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetCSMAgentlessHostFacetInfo", true);
    let api = CSMSettingsAPI::with_config(configuration);
    let resp = api
        .get_csm_agentless_host_facet_info(
            "cloud_provider".to_string(),
            GetCSMAgentlessHostFacetInfoOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
