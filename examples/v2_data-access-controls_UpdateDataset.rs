// Edit a Data Access Control dataset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_data_access_controls::DataAccessControlsAPI;
use datadog_api_client::datadogV2::model::DatasetAttributesRequest;
use datadog_api_client::datadogV2::model::DatasetRequest;
use datadog_api_client::datadogV2::model::DatasetType;
use datadog_api_client::datadogV2::model::DatasetUpdateRequest;
use datadog_api_client::datadogV2::model::FiltersPerProduct;

#[tokio::main]
async fn main() {
    // there is a valid "dataset" in the system
    let dataset_data_id = std::env::var("DATASET_DATA_ID").unwrap();
    let body = DatasetUpdateRequest::new(DatasetRequest::new(
        DatasetAttributesRequest::new(
            "Security Audit DAC".to_string(),
            vec!["role:94172442-be03-11e9-a77a-3b7612558ac1".to_string()],
            vec![FiltersPerProduct::new(
                vec!["@application.id:1234".to_string()],
                "logs".to_string(),
            )],
        ),
        DatasetType::DATASET,
    ));
    let configuration = datadog::Configuration::new();
    let api = DataAccessControlsAPI::with_config(configuration);
    let resp = api.update_dataset(dataset_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
