// Get code coverage summary for a commit returns "OK" response
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
            "66adc9350f2cc9b250b69abddab733dd55e1a588".to_string(),
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
