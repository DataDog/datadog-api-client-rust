// Update a restriction query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_restriction_queries::LogsRestrictionQueriesAPI;
use datadog_api_client::datadogV2::model::LogsRestrictionQueriesType;
use datadog_api_client::datadogV2::model::RestrictionQueryUpdateAttributes;
use datadog_api_client::datadogV2::model::RestrictionQueryUpdateData;
use datadog_api_client::datadogV2::model::RestrictionQueryUpdatePayload;

#[tokio::main]
async fn main() {
    let body = RestrictionQueryUpdatePayload::new().data(
        RestrictionQueryUpdateData::new()
            .attributes(
                RestrictionQueryUpdateAttributes::new()
                    .restriction_query("env:sandbox".to_string()),
            )
            .type_(LogsRestrictionQueriesType::LOGS_RESTRICTION_QUERIES),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateRestrictionQuery", true);
    let api = LogsRestrictionQueriesAPI::with_config(configuration);
    let resp = api
        .update_restriction_query("restriction_query_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
