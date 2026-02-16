// Get code coverage summary for an existing commit with valid repository
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_code_coverage::CodeCoverageAPI;
use datadog_api_client::datadogV2::model::CommitCoverageSummaryRequest;
use datadog_api_client::datadogV2::model::CommitCoverageSummaryRequestAttributes;
use datadog_api_client::datadogV2::model::CommitCoverageSummaryRequestData;
use datadog_api_client::datadogV2::model::CommitCoverageSummaryRequestType;

#[tokio::main]
async fn main() {
    let body = CommitCoverageSummaryRequest::new(CommitCoverageSummaryRequestData::new(
        CommitCoverageSummaryRequestAttributes::new(
            "c55b0ce584e139bde41a00002ab31bc7d75f791d".to_string(),
            "github.com/datadog/shopist".to_string(),
        ),
        CommitCoverageSummaryRequestType::CI_APP_COVERAGE_COMMIT_SUMMARY_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetCodeCoverageCommitSummary", true);
    let api = CodeCoverageAPI::with_config(configuration);
    let resp = api.get_code_coverage_commit_summary(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
