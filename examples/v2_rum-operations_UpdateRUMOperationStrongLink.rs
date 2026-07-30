// Update a RUM operation strong link returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkType;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkUpdateRequest;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkUpdateRequestData;
use datadog_api_client::datadogV2::model::RUMOperationStrongLinkUpdateStatus;

#[tokio::main]
async fn main() {
    let body =
        RUMOperationStrongLinkUpdateRequest::new(RUMOperationStrongLinkUpdateRequestData::new(
            RUMOperationStrongLinkUpdateRequestAttributes::new(
                RUMOperationStrongLinkUpdateStatus::CONFIRMED,
            ),
            RUMOperationStrongLinkType::STRONG_LINKS,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateRUMOperationStrongLink", true);
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api
        .update_rum_operation_strong_link(
            "rum_operation_id".to_string(),
            "feature_id".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
