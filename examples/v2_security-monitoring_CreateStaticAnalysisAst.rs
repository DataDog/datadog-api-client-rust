// Get AST for source code returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::GetAstRequest;
use datadog_api_client::datadogV2::model::GetAstRequestData;
use datadog_api_client::datadogV2::model::GetAstRequestDataAttributes;
use datadog_api_client::datadogV2::model::GetAstRequestDataType;

#[tokio::main]
async fn main() {
    let body = GetAstRequest::new(GetAstRequestData::new(
        GetAstRequestDataAttributes::new(
            "aW1wb3J0IHN5cw==".to_string(),
            "utf-8".to_string(),
            "python".to_string(),
        ),
        GetAstRequestDataType::GET_AST_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateStaticAnalysisAst", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_static_analysis_ast(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
