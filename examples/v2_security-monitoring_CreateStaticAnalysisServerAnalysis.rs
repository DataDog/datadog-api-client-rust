// Analyze code returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AnalysisRequest;
use datadog_api_client::datadogV2::model::AnalysisRequestData;
use datadog_api_client::datadogV2::model::AnalysisRequestDataAttributes;
use datadog_api_client::datadogV2::model::AnalysisRequestDataType;
use datadog_api_client::datadogV2::model::AnalysisRequestRule;

#[tokio::main]
async fn main() {
    let body = AnalysisRequest::new(AnalysisRequestData::new(
        AnalysisRequestDataAttributes::new(
            "aW1wb3J0IHN5cw==".to_string(),
            "utf-8".to_string(),
            "test.py".to_string(),
            "python".to_string(),
            vec![AnalysisRequestRule::new(
                "BEST_PRACTICES".to_string(),
                "abc123def456".to_string(),
                "ZnVuY3Rpb24gdmlzaXQobm9kZSkge30=".to_string(),
                "python-best-practices/no-exit".to_string(),
                "python".to_string(),
                "WARNING".to_string(),
                "KGNhbGwgbmFtZTogKGF0dHJpYnV0ZSkpQHZhbA==".to_string(),
                "TREE_SITTER_QUERY".to_string(),
            )
            .entity_checked(None)
            .regex(None)],
        ),
        AnalysisRequestDataType::ANALYSIS_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateStaticAnalysisServerAnalysis", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_static_analysis_server_analysis(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
