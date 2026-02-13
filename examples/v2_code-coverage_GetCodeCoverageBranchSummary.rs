// Get code coverage summary for a branch returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_code_coverage::CodeCoverageAPI;
use datadog_api_client::datadogV2::model::BranchCoverageSummaryRequest;
use datadog_api_client::datadogV2::model::BranchCoverageSummaryRequestAttributes;
use datadog_api_client::datadogV2::model::BranchCoverageSummaryRequestData;
use datadog_api_client::datadogV2::model::BranchCoverageSummaryRequestType;

#[tokio::main]
async fn main() {
    let body = BranchCoverageSummaryRequest::new(BranchCoverageSummaryRequestData::new(
        BranchCoverageSummaryRequestAttributes::new(
            "prod".to_string(),
            "github.com/datadog/shopist".to_string(),
        ),
        BranchCoverageSummaryRequestType::CI_APP_COVERAGE_BRANCH_SUMMARY_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetCodeCoverageBranchSummary", true);
    let api = CodeCoverageAPI::with_config(configuration);
    let resp = api.get_code_coverage_branch_summary(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
