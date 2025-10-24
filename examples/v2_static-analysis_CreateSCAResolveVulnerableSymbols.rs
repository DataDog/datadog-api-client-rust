// POST request to resolve vulnerable symbols returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::ResolveVulnerableSymbolsRequest;
use datadog_api_client::datadogV2::model::ResolveVulnerableSymbolsRequestData;
use datadog_api_client::datadogV2::model::ResolveVulnerableSymbolsRequestDataAttributes;
use datadog_api_client::datadogV2::model::ResolveVulnerableSymbolsRequestDataType;

#[tokio::main]
async fn main() {
    let body = ResolveVulnerableSymbolsRequest::new().data(
        ResolveVulnerableSymbolsRequestData::new(
            ResolveVulnerableSymbolsRequestDataType::RESOLVE_VULNERABLE_SYMBOLS_REQUEST,
        )
        .attributes(ResolveVulnerableSymbolsRequestDataAttributes::new().purls(vec![])),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateSCAResolveVulnerableSymbols", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api.create_sca_resolve_vulnerable_symbols(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
