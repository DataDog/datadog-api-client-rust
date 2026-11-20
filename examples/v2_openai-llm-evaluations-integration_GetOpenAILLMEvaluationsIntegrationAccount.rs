// Get an OpenAI LLM Evaluations account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_aillm_evaluations_integration::OpenAILLMEvaluationsIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.get_open_aillm_evaluations_integration_account", true);
    let api = OpenAILLMEvaluationsIntegrationAPI::with_config(configuration);
    let resp = api
        .get_open_aillm_evaluations_integration_account("account_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
