// Update a ChatGPT Enterprise integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_chat_gpt_enterprise_integration::ChatGPTEnterpriseIntegrationAPI;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseCloudCostDataIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountBearerTokenAuthUpdate;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::ChatGPTEnterpriseIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        ChatGPTEnterpriseIntegrationAccountUpdateRequest::new(
            ChatGPTEnterpriseIntegrationAccountUpdateData::new(
                ChatGPTEnterpriseIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        ChatGPTEnterpriseIntegrationAccountAuthenticationUpdate
                        ::ChatGPTEnterpriseIntegrationAccountBearerTokenAuthUpdate(
                            Box::new(
                                ChatGPTEnterpriseIntegrationAccountBearerTokenAuthUpdate::new(
                                    ChatGPTEnterpriseIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                ).token("your-token".to_string()),
                            ),
                        ),
                    )
                    .dataflows(
                        ChatGPTEnterpriseIntegrationDataflowsRequest::new()
                            .chatgpt_enterprise_agent_console(
                                ChatGPTEnterpriseAgentConsoleIntegrationDataflowRequest::new(true),
                            )
                            .chatgpt_enterprise_cloud_cost_data(
                                ChatGPTEnterpriseCloudCostDataIntegrationDataflowRequest::new().enabled(true),
                            ),
                    )
                    .name("chatgpt-enterprise-acme".to_string())
                    .settings(
                        ChatGPTEnterpriseIntegrationAccountSettingsUpdate
                        ::new().openai_org_id("org-AbC123dEf456".to_string()),
                    ),
                "3e8a1f6b-9c2d-4b7e-8a5f-6d1c4e9b2a03".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_chat_gpt_enterprise_integration_account", true);
    let api = ChatGPTEnterpriseIntegrationAPI::with_config(configuration);
    let resp = api
        .update_chat_gpt_enterprise_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
