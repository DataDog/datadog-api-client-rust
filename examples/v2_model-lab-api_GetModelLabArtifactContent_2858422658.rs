// Download artifact content returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_model_lab_api::ModelLabAPIAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetModelLabArtifactContent", true);
    let api = ModelLabAPIAPI::with_config(configuration);
    let resp = api
        .get_model_lab_artifact_content(
            "2387".to_string(),
            "f635c73b70594ab6bb6e212cdf87d0d5/artifacts/lora_model/adapter_config.json".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
