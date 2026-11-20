// Create an OpenAI Usage and Costs account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_ai_usageand_costs_integration::OpenAIUsageandCostsIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsCloudCostDataIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthRequest;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationDataflowsRequest;

#[tokio::main]
async fn main() {
    let body =
        OpenAIUsageAndCostsIntegrationAccountCreateRequest::new(
            OpenAIUsageAndCostsIntegrationAccountCreateData::new(
                OpenAIUsageAndCostsIntegrationAccountCreateAttributes::new(
                    OpenAIUsageAndCostsIntegrationAccountAuthenticationRequest
                    ::OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthRequest(
                        Box::new(
                            OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthRequest::new(
                                OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                "your-token".to_string(),
                            ),
                        ),
                    ),
                    "openai-usage-and-costs-acme".to_string(),
                    OpenAIUsageAndCostsIntegrationAccountSettingsRequest
                    ::new().custom_tags("team:ai,env:prod".to_string()),
                ).dataflows(
                    OpenAIUsageAndCostsIntegrationDataflowsRequest
                    ::new().openai_cloud_cost_data(
                        OpenAIUsageAndCostsCloudCostDataIntegrationDataflowRequest::new().enabled(true),
                    ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.create_open_ai_usage_and_costs_integration_account",
        true,
    );
    let api = OpenAIUsageandCostsIntegrationAPI::with_config(configuration);
    let resp = api
        .create_open_ai_usage_and_costs_integration_account(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
