// Fetch the result of a DDSQL query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ddsql::DDSQLAPI;
use datadog_api_client::datadogV2::model::DdsqlTabularQueryFetchRequest;
use datadog_api_client::datadogV2::model::DdsqlTabularQueryFetchRequestAttributes;
use datadog_api_client::datadogV2::model::DdsqlTabularQueryFetchRequestData;
use datadog_api_client::datadogV2::model::DdsqlTabularQueryFetchRequestType;

#[tokio::main]
async fn main() {
    let body = DdsqlTabularQueryFetchRequest::new(DdsqlTabularQueryFetchRequestData::new(
        DdsqlTabularQueryFetchRequestAttributes::new(
            "eyJxdWVyeSI6ICJTRUxFQ1QgKiBGUk9NIGxvZ3MifQ==".to_string(),
        ),
        DdsqlTabularQueryFetchRequestType::DDSQL_QUERY_FETCH_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = DDSQLAPI::with_config(configuration);
    let resp = api.fetch_ddsql_tabular_query(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
