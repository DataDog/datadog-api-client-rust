// Update a restriction query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_restriction_queries::LogsRestrictionQueriesAPI;
use datadog_api_client::datadogV2::model::LogsRestrictionQueriesType;
use datadog_api_client::datadogV2::model::RestrictionQueryUpdateAttributes;
use datadog_api_client::datadogV2::model::RestrictionQueryUpdateData;
use datadog_api_client::datadogV2::model::RestrictionQueryUpdatePayload;

#[tokio::main]
async fn main() {
    // there is a valid "restriction_query" in the system
    let restriction_query_data_id = std::env::var("RESTRICTION_QUERY_DATA_ID").unwrap();
    let body = RestrictionQueryUpdatePayload::new().data(
        RestrictionQueryUpdateData::new()
            .attributes(RestrictionQueryUpdateAttributes::new(
                "env:production".to_string(),
            ))
            .type_(LogsRestrictionQueriesType::LOGS_RESTRICTION_QUERIES),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateRestrictionQuery", true);
    let api = LogsRestrictionQueriesAPI::with_config(configuration);
    let resp = api
        .update_restriction_query(restriction_query_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
