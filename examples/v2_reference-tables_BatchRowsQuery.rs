// Batch rows query returns "Successfully retrieved rows. Some or all requested
// rows were found. Response includes found rows in the included section." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_reference_tables::ReferenceTablesAPI;
use datadog_api_client::datadogV2::model::BatchRowsQueryDataType;
use datadog_api_client::datadogV2::model::BatchRowsQueryRequest;
use datadog_api_client::datadogV2::model::BatchRowsQueryRequestData;
use datadog_api_client::datadogV2::model::BatchRowsQueryRequestDataAttributes;

#[tokio::main]
async fn main() {
    let body = BatchRowsQueryRequest::new().data(
        BatchRowsQueryRequestData::new(BatchRowsQueryDataType::REFERENCE_TABLES_BATCH_ROWS_QUERY)
            .attributes(BatchRowsQueryRequestDataAttributes::new(
                vec!["row_id_1".to_string(), "row_id_2".to_string()],
                "00000000-0000-0000-0000-000000000000".to_string(),
            )),
    );
    let configuration = datadog::Configuration::new();
    let api = ReferenceTablesAPI::with_config(configuration);
    let resp = api.batch_rows_query(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
