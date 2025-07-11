// Create a dataset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_datasets::DatasetsAPI;
use datadog_api_client::datadogV2::model::Dataset;
use datadog_api_client::datadogV2::model::DatasetAttributes;
use datadog_api_client::datadogV2::model::DatasetCreateRequest;
use datadog_api_client::datadogV2::model::FiltersPerProduct;

#[tokio::main]
async fn main() {
    let body = DatasetCreateRequest::new(
        Dataset::new(
            DatasetAttributes::new(
                "Security Audit Dataset".to_string(),
                vec!["role:86245fce-0a4e-11f0-92bd-da7ad0900002".to_string()],
                vec![FiltersPerProduct::new(
                    vec!["@application.id:ABCD".to_string()],
                    "logs".to_string(),
                )],
            )
            .created_at(None),
            "dataset".to_string(),
        )
        .id("123e4567-e89b-12d3-a456-426614174000".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = DatasetsAPI::with_config(configuration);
    let resp = api.create_dataset(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
