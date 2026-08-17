// Get code coverage summary for a pull request returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_code_coverage::CodeCoverageAPI;
use datadog_api_client::datadogV2::model::PRCoverageSummaryRequest;
use datadog_api_client::datadogV2::model::PRCoverageSummaryRequestAttributes;
use datadog_api_client::datadogV2::model::PRCoverageSummaryRequestData;
use datadog_api_client::datadogV2::model::PRCoverageSummaryRequestType;

#[tokio::main]
async fn main() {
    let body = PRCoverageSummaryRequest::new(PRCoverageSummaryRequestData::new(
        PRCoverageSummaryRequestAttributes::new(
            42,
            "https://github.com/datadog/shopist".to_string(),
        ),
        PRCoverageSummaryRequestType::CI_APP_COVERAGE_PR_SUMMARY_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = CodeCoverageAPI::with_config(configuration);
    let resp = api.get_code_coverage_pr_summary(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
