// Delete a RUM operation link returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "rum_operation_strong_link" in the system
    let rum_operation_strong_link_data_attributes_feature_id =
        std::env::var("RUM_OPERATION_STRONG_LINK_DATA_ATTRIBUTES_FEATURE_ID").unwrap();
    let rum_operation_strong_link_data_attributes_operation_id =
        std::env::var("RUM_OPERATION_STRONG_LINK_DATA_ATTRIBUTES_OPERATION_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api
        .delete_rum_operation_strong_link(
            rum_operation_strong_link_data_attributes_operation_id.clone(),
            rum_operation_strong_link_data_attributes_feature_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
