// List ASM Exclusion Filters returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_asm_exclusion_filters::ASMExclusionFiltersAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ASMExclusionFiltersAPI::with_config(configuration);
    let resp = api.list_asm_exclusion_filters().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
