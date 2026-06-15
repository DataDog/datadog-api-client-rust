// Create or update a patterns configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsPatternsConfigType;
use datadog_api_client::datadogV2::model::LLMObsPatternsConfigUpsertRequest;
use datadog_api_client::datadogV2::model::LLMObsPatternsConfigUpsertRequestAttributes;
use datadog_api_client::datadogV2::model::LLMObsPatternsConfigUpsertRequestData;

#[tokio::main]
async fn main() {
    let body = LLMObsPatternsConfigUpsertRequest::new(LLMObsPatternsConfigUpsertRequestData::new(
        LLMObsPatternsConfigUpsertRequestAttributes::new(
            "@ml_app:support-bot".to_string(),
            2,
            "Support chatbot topics".to_string(),
            1000,
            0.1,
        )
        .account_id("1000000001".to_string())
        .config_id("a7c8d9e0-1234-5678-9abc-def012345678".to_string())
        .integration_provider("openai".to_string())
        .model_name("gpt-4o".to_string())
        .scope("".to_string())
        .template("".to_string()),
        LLMObsPatternsConfigType::TOPIC_DISCOVERY_CONFIGS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpsertLLMObsPatternsConfig", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.upsert_llm_obs_patterns_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
