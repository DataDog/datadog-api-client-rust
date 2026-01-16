// Synthetics: Bulk delete suites returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::DeletedSuitesRequestDelete;
use datadog_api_client::datadogV2::model::DeletedSuitesRequestDeleteAttributes;
use datadog_api_client::datadogV2::model::DeletedSuitesRequestDeleteRequest;
use datadog_api_client::datadogV2::model::DeletedSuitesRequestType;

#[tokio::main]
async fn main() {
    let body = DeletedSuitesRequestDeleteRequest::new(
        DeletedSuitesRequestDelete::new(DeletedSuitesRequestDeleteAttributes::new(vec![
            "".to_string()
        ]))
        .type_(DeletedSuitesRequestType::DELETE_SUITES_REQUEST),
    );
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.delete_synthetics_suites(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
