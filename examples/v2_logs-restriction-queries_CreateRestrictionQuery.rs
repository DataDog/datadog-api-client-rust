// Create a restriction query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_restriction_queries::LogsRestrictionQueriesAPI;
use datadog_api_client::datadogV2::model::LogsRestrictionQueriesType;
use datadog_api_client::datadogV2::model::RestrictionQueryCreateAttributes;
use datadog_api_client::datadogV2::model::RestrictionQueryCreateData;
use datadog_api_client::datadogV2::model::RestrictionQueryCreatePayload;

#[tokio::main]
async fn main() {
    let body = RestrictionQueryCreatePayload::new().data(
        RestrictionQueryCreateData::new()
            .attributes(RestrictionQueryCreateAttributes::new(
                "env:sandbox".to_string(),
            ))
            .type_(LogsRestrictionQueriesType::LOGS_RESTRICTION_QUERIES),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateRestrictionQuery", true);
    let api = LogsRestrictionQueriesAPI::with_config(configuration);
    let resp = api.create_restriction_query(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
