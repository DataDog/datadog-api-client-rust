// Delete LLM Observability data returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDataDeletionRequest;
use datadog_api_client::datadogV2::model::LLMObsDataDeletionRequestAttributes;
use datadog_api_client::datadogV2::model::LLMObsDataDeletionRequestData;
use datadog_api_client::datadogV2::model::LLMObsDataDeletionRequestType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = LLMObsDataDeletionRequest::new(LLMObsDataDeletionRequestData::new(
        LLMObsDataDeletionRequestAttributes::new(
            1705314600000,
            BTreeMap::from([("query".to_string(), "@trace_id:abc123def456".to_string())]),
            1705315200000,
        )
        .delay(0),
        LLMObsDataDeletionRequestType::CREATE_DELETION_REQ,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteLLMObsData", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.delete_llm_obs_data(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
