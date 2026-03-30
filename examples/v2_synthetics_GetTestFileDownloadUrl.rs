// Get a presigned URL for downloading a test file returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::SyntheticsTestFileDownloadRequest;

#[tokio::main]
async fn main() {
    let body = SyntheticsTestFileDownloadRequest::new(
        "api-upload-file/abc-def-123/2024-01-01T00:00:00_uuid.json".to_string(),
    );
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .get_test_file_download_url("abc-def-123".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
