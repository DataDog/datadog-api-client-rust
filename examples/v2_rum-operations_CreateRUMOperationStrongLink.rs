// Create a RUM operation link returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkCreateRequest;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkCreateRequestAttributes;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkCreateRequestData;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkStatus;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkType;

#[tokio::main]
async fn main() {
    // there is a valid "rum_operation" in the system
    let rum_operation_data_id = std::env::var("RUM_OPERATION_DATA_ID").unwrap();
    let body =
        RUMOperationStrongLinkCreateRequest::new(RUMOperationStrongLinkCreateRequestData::new(
            RUMOperationStrongLinkCreateRequestAttributes::new(
                "bdd-examplerumoperation".to_string(),
            )
            .operation_id(rum_operation_data_id.clone())
            .status(RUMOperationStrongLinkStatus::DRAFT)
            .tags(vec!["team:rum-backend".to_string()]),
            RUMOperationStrongLinkType::STRONG_LINKS,
        ));
    let configuration = datadog::Configuration::new();
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api.create_rum_operation_strong_link(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
