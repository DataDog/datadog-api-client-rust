// Create a ChatGPT Enterprise integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_chat_gpt_enterprise_integration::ChatGPTEnterpriseIntegrationAPI;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseCloudCostDataIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountBearerTokenAuthRequest;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        ChatGPTEnterpriseIntegrationAccountCreateRequest::new(
            ChatGPTEnterpriseIntegrationAccountCreateData::new(
                ChatGPTEnterpriseIntegrationAccountCreateAttributes::new(
                    ChatGPTEnterpriseIntegrationAccountAuthenticationRequest
                    ::ChatGPTEnterpriseIntegrationAccountBearerTokenAuthRequest(
                        Box::new(
                            ChatGPTEnterpriseIntegrationAccountBearerTokenAuthRequest::new(
                                ChatGPTEnterpriseIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                "your-token".to_string(),
                            ),
                        ),
                    ),
                    "chatgpt-enterprise-acme".to_string(),
                    ChatGPTEnterpriseIntegrationAccountSettingsRequest::new("org-AbC123dEf456".to_string()),
                ).dataflows(
                    ChatGPTEnterpriseIntegrationDataflowsRequest::new()
                        .chatgpt_enterprise_agent_console(
                            ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest::new(true),
                        )
                        .chatgpt_enterprise_cloud_cost_data(
                            ChatGPTEnterpriseCloudCostDataIntegrationDataflowRequest::new().enabled(true),
                        ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.create_chat_gpt_enterprise_integration_account", true);
    let api = ChatGPTEnterpriseIntegrationAPI::with_config(configuration);
    let resp = api
        .create_chat_gpt_enterprise_integration_account(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
