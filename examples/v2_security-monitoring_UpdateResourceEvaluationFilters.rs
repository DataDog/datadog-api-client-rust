// Update resource filters returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::ResourceFilterAttributes;
use datadog_api_client::datadogV2::model::ResourceFilterRequestType;
use datadog_api_client::datadogV2::model::UpdateResourceEvaluationFiltersRequest;
use datadog_api_client::datadogV2::model::UpdateResourceEvaluationFiltersRequestData;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = UpdateResourceEvaluationFiltersRequest::new(
        UpdateResourceEvaluationFiltersRequestData::new(
            ResourceFilterAttributes::new(BTreeMap::from([(
                "aws".to_string(),
                BTreeMap::from([("aws_account_id".to_string(), vec!["tag1:v1".to_string()])]),
            )])),
            ResourceFilterRequestType::CSM_RESOURCE_FILTER,
        )
        .id("csm_resource_filter".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.update_resource_evaluation_filters(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
