// Submit libraries for vulnerability scanning returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::McpScanRequest;
use datadog_api_client::datadogV2::model::McpScanRequestData;
use datadog_api_client::datadogV2::model::McpScanRequestDataAttributes;
use datadog_api_client::datadogV2::model::McpScanRequestDataAttributesLibrariesItems;
use datadog_api_client::datadogV2::model::McpScanRequestDataType;

#[tokio::main]
async fn main() {
    let body = McpScanRequest::new(McpScanRequestData::new(
        McpScanRequestDataAttributes::new(
            "0e9fc8de83eaabecd722e1cd0ed44fb489fe15fc".to_string(),
            vec![McpScanRequestDataAttributesLibrariesItems::new(
                false,
                true,
                "nuget".to_string(),
                "pkg:nuget/Newtonsoft.Json@13.0.1".to_string(),
            )
            .exclusions(vec![])
            .target_frameworks(vec![])],
            "my-org/my-repo".to_string(),
        ),
        McpScanRequestDataType::MCPSCANREQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateSCAScan", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api.create_sca_scan(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
