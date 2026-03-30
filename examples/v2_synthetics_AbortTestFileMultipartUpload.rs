// Abort a multipart upload of a test file returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::SyntheticsTestFileAbortMultipartUploadRequest;

#[tokio::main]
async fn main() {
    let body = SyntheticsTestFileAbortMultipartUploadRequest::new(
        "org-123/api-upload-file/abc-def-123/2024-01-01T00:00:00_uuid.json".to_string(),
        "upload-id-abc123".to_string(),
    );
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .abort_test_file_multipart_upload("abc-def-123".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
