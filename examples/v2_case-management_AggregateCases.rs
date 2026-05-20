// Aggregate cases returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseAggregateGroupBy;
use datadog_api_client::datadogV2::model::CaseAggregateRequest;
use datadog_api_client::datadogV2::model::CaseAggregateRequestAttributes;
use datadog_api_client::datadogV2::model::CaseAggregateRequestData;
use datadog_api_client::datadogV2::model::CaseAggregateResourceType;

#[tokio::main]
async fn main() {
    let body = CaseAggregateRequest::new(CaseAggregateRequestData::new(
        CaseAggregateRequestAttributes::new(
            CaseAggregateGroupBy::new(vec!["status".to_string()], 14),
            "service:case-api".to_string(),
        ),
        CaseAggregateResourceType::AGGREGATE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.AggregateCases", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.aggregate_cases(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
