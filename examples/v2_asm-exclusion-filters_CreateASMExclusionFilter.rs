// Create a ASM WAF Exclusion filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_asm_exclusion_filters::ASMExclusionFiltersAPI;
use datadog_api_client::datadogV2::model::ASMExclusionFilterCreateAttributes;
use datadog_api_client::datadogV2::model::ASMExclusionFilterCreateData;
use datadog_api_client::datadogV2::model::ASMExclusionFilterCreateRequest;
use datadog_api_client::datadogV2::model::ASMExclusionFilterRulesTarget;
use datadog_api_client::datadogV2::model::ASMExclusionFilterScope;
use datadog_api_client::datadogV2::model::ASMExclusionFilterType;

#[tokio::main]
async fn main() {
    let body = ASMExclusionFilterCreateRequest::new(ASMExclusionFilterCreateData::new(
        ASMExclusionFilterCreateAttributes::new(
            "my description".to_string(),
            true,
            "*".to_string(),
        )
        .rules_target(vec![ASMExclusionFilterRulesTarget::new()])
        .scope(vec![ASMExclusionFilterScope::new()
            .env("staging".to_string())
            .service("container-resolver".to_string())]),
        ASMExclusionFilterType::EXCLUSION_FILTER,
    ));
    let configuration = datadog::Configuration::new();
    let api = ASMExclusionFiltersAPI::with_config(configuration);
    let resp = api.create_asm_exclusion_filter(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
