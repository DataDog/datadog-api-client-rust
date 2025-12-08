// Upsert rows returns "Rows created or updated successfully" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_reference_tables::ReferenceTablesAPI;
use datadog_api_client::datadogV2::model::BatchUpsertRowsRequestArray;
use datadog_api_client::datadogV2::model::BatchUpsertRowsRequestData;
use datadog_api_client::datadogV2::model::BatchUpsertRowsRequestDataAttributes;
use datadog_api_client::datadogV2::model::TableRowResourceDataType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = BatchUpsertRowsRequestArray::new(vec![BatchUpsertRowsRequestData::new(
        "primary_key_value".to_string(),
        TableRowResourceDataType::ROW,
    )
    .attributes(BatchUpsertRowsRequestDataAttributes::new(BTreeMap::from([
        (
            "example_key_value".to_string(),
            Value::from("primary_key_value"),
        ),
        ("name".to_string(), Value::from("row_name")),
    ])))]);
    let configuration = datadog::Configuration::new();
    let api = ReferenceTablesAPI::with_config(configuration);
    let resp = api.upsert_rows("id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
