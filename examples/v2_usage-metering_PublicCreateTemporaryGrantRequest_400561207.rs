// Create a temporary grant request with a justification
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestCreateAttributes;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestCreateData;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestCreateRequest;
use datadog_api_client::datadogV2::model::TemporaryGrantRequestType;

#[tokio::main]
async fn main() {
    let body = TemporaryGrantRequestCreateRequest::new(TemporaryGrantRequestCreateData::new(
        TemporaryGrantRequestCreateAttributes::new(
            "SDK temporary grant request test".to_string(),
            200,
        ),
        TemporaryGrantRequestType::TEMPORARY_GRANT_REQUESTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.public_create_temporary_grant_request", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api.public_create_temporary_grant_request(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
