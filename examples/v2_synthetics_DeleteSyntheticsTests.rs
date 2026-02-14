// Synthetics: Bulk delete tests returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::DeletedTestsRequestDelete;
use datadog_api_client::datadogV2::model::DeletedTestsRequestDeleteAttributes;
use datadog_api_client::datadogV2::model::DeletedTestsRequestDeleteRequest;
use datadog_api_client::datadogV2::model::DeletedTestsRequestType;

#[tokio::main]
async fn main() {
    let body = DeletedTestsRequestDeleteRequest::new(
        DeletedTestsRequestDelete::new(DeletedTestsRequestDeleteAttributes::new(vec![
            "".to_string()
        ]))
        .type_(DeletedTestsRequestType::DELETE_TESTS_REQUEST),
    );
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.delete_synthetics_tests(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
