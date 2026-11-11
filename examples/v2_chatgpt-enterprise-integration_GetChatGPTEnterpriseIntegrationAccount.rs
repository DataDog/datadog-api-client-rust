// Get a ChatGPT Enterprise integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_chat_gpt_enterprise_integration::ChatGPTEnterpriseIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.get_chat_gpt_enterprise_integration_account", true);
    let api = ChatGPTEnterpriseIntegrationAPI::with_config(configuration);
    let resp = api
        .get_chat_gpt_enterprise_integration_account("account_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
