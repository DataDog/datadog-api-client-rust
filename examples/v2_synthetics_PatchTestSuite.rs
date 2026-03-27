// Patch a test suite returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::JsonPatchOperation;
use datadog_api_client::datadogV2::model::JsonPatchOperationOp;
use datadog_api_client::datadogV2::model::SuiteJsonPatchRequest;
use datadog_api_client::datadogV2::model::SuiteJsonPatchRequestData;
use datadog_api_client::datadogV2::model::SuiteJsonPatchRequestDataAttributes;
use datadog_api_client::datadogV2::model::SuiteJsonPatchType;

#[tokio::main]
async fn main() {
    let body = SuiteJsonPatchRequest::new(
        SuiteJsonPatchRequestData::new()
            .attributes(SuiteJsonPatchRequestDataAttributes::new().json_patch(vec![
                JsonPatchOperation::new(JsonPatchOperationOp::ADD, "/name".to_string()),
            ]))
            .type_(SuiteJsonPatchType::SUITES_JSON_PATCH),
    );
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.patch_test_suite("123-abc-456".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
