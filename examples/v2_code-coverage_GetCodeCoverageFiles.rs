// Get per-file code coverage data returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_code_coverage::CodeCoverageAPI;
use datadog_api_client::datadogV2::model::FilesCoverageRequest;
use datadog_api_client::datadogV2::model::FilesCoverageRequestAttributes;
use datadog_api_client::datadogV2::model::FilesCoverageRequestData;
use datadog_api_client::datadogV2::model::FilesCoverageRequestType;

#[tokio::main]
async fn main() {
    let body = FilesCoverageRequest::new(FilesCoverageRequestData::new(
        FilesCoverageRequestAttributes::new()
            .changed_only(true)
            .commit_sha("66adc9350f2cc9b250b69abddab733dd55e1a588".to_string())
            .repository_url("https://github.com/datadog/shopist".to_string()),
        FilesCoverageRequestType::CI_APP_COVERAGE_FILES_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = CodeCoverageAPI::with_config(configuration);
    let resp = api.get_code_coverage_files(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
