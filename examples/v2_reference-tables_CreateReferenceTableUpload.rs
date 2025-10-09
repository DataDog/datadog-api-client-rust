// Create reference table upload returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_reference_tables::ReferenceTablesAPI;
use datadog_api_client::datadogV2::model::CreateUploadRequest;
use datadog_api_client::datadogV2::model::CreateUploadRequestData;
use datadog_api_client::datadogV2::model::CreateUploadRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateUploadRequestDataType;

#[tokio::main]
async fn main() {
    let body = CreateUploadRequest::new().data(
        CreateUploadRequestData::new(CreateUploadRequestDataType::UPLOAD).attributes(
            CreateUploadRequestDataAttributes::new(
                vec!["id".to_string(), "name".to_string(), "value".to_string()],
                1,
                1024,
                "test_upload_table_Example-Reference-Table".to_string(),
            ),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ReferenceTablesAPI::with_config(configuration);
    let resp = api.create_reference_table_upload(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
