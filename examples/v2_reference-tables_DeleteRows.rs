// Delete rows returns "Rows deleted successfully" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_reference_tables::ReferenceTablesAPI;
use datadog_api_client::datadogV2::model::BatchDeleteRowsRequestArray;
use datadog_api_client::datadogV2::model::BatchDeleteRowsRequestData;
use datadog_api_client::datadogV2::model::TableRowResourceDataType;

#[tokio::main]
async fn main() {
    let body = BatchDeleteRowsRequestArray::new(vec![BatchDeleteRowsRequestData::new(
        "primary_key_value".to_string(),
        TableRowResourceDataType::ROW,
    )]);
    let configuration = datadog::Configuration::new();
    let api = ReferenceTablesAPI::with_config(configuration);
    let resp = api.delete_rows("id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
