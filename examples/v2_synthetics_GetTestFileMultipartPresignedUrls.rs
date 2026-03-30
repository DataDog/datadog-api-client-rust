// Get presigned URLs for uploading a test file returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsPart;
use datadog_api_client::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsRequest;
use datadog_api_client::datadogV2::model::SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix;

#[tokio::main]
async fn main() {
    let body = SyntheticsTestFileMultipartPresignedUrlsRequest::new(
        SyntheticsTestFileMultipartPresignedUrlsRequestBucketKeyPrefix::API_UPLOAD_FILE,
        vec![SyntheticsTestFileMultipartPresignedUrlsPart::new(
            "1B2M2Y8AsgTpgAmY7PhCfg==".to_string(),
            1,
        )],
    );
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .get_test_file_multipart_presigned_urls("abc-def-123".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
