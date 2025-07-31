// Create a dataset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_datasets::DatasetsAPI;
use datadog_api_client::datadogV2::model::DatasetAttributesRequest;
use datadog_api_client::datadogV2::model::DatasetCreateRequest;
use datadog_api_client::datadogV2::model::DatasetRequest;
use datadog_api_client::datadogV2::model::FiltersPerProduct;

#[tokio::main]
async fn main() {
    let body = DatasetCreateRequest::new(DatasetRequest::new(
        DatasetAttributesRequest::new(
            "Security Audit Dataset".to_string(),
            vec!["role:94172442-be03-11e9-a77a-3b7612558ac1".to_string()],
            vec![FiltersPerProduct::new(
                vec!["@application.id:ABCD".to_string()],
                "metrics".to_string(),
            )],
        ),
        "dataset".to_string(),
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateDataset", true);
    let api = DatasetsAPI::with_config(configuration);
    let resp = api.create_dataset(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
