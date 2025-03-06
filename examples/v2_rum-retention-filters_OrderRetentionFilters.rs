// Order RUM retention filters returns "Ordered" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RumRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RumRetentionFilterType;
use datadog_api_client::datadogV2::model::RumRetentionFiltersOrderData;
use datadog_api_client::datadogV2::model::RumRetentionFiltersOrderRequest;

#[tokio::main]
async fn main() {
    let body = RumRetentionFiltersOrderRequest::new().data(vec![
        RumRetentionFiltersOrderData::new(
            "325631eb-94c9-49c0-93f9-ab7e4fd24529".to_string(),
            RumRetentionFilterType::RETENTION_FILTERS,
        ),
        RumRetentionFiltersOrderData::new(
            "42d89430-5b80-426e-a44b-ba3b417ece25".to_string(),
            RumRetentionFilterType::RETENTION_FILTERS,
        ),
        RumRetentionFiltersOrderData::new(
            "bff0bc34-99e9-4c16-adce-f47e71948c23".to_string(),
            RumRetentionFilterType::RETENTION_FILTERS,
        ),
    ]);
    let configuration = datadog::Configuration::new();
    let api = RumRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .order_retention_filters("1d4b9c34-7ac4-423a-91cf-9902d926e9b3".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
