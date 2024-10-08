// Get a specific ASM Exclusion Filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_asm_exclusion_filters::ASMExclusionFiltersAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ASMExclusionFiltersAPI::with_config(configuration);
    let resp = api
        .get_asm_exclusion_filters("exclusion_filter_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
