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
        UpdateResourceEvaluationFiltersRequestData::new()
            .attributes(ResourceFilterAttributes::new(BTreeMap::from([(
                "aws".to_string(),
                BTreeMap::from([("123456789".to_string(), vec![])]),
            )])))
            .id("csm_resource_filter".to_string())
            .type_(ResourceFilterRequestType::CSM_RESOURCE_FILTER),
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
