// Create a RUM operation strong link returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkCreateRequest;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkCreateRequestAttributes;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkCreateRequestData;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkStatus;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkType;

#[tokio::main]
async fn main() {
    let body =
        RUMOperationStrongLinkCreateRequest::new(RUMOperationStrongLinkCreateRequestData::new(
            RUMOperationStrongLinkCreateRequestAttributes::new("feature-123".to_string())
                .description(None)
                .operation_id("abc12345-1234-5678-abcd-ef1234567890".to_string())
                .status(RUMOperationStrongLinkStatus::CONFIRMED)
                .tags(vec![]),
            RUMOperationStrongLinkType::STRONG_LINKS,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateRUMOperationStrongLink", true);
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api.create_rum_operation_strong_link(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
