// Update an OpenAI Usage and Costs account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_ai_usageand_costs_integration::OpenAIUsageandCostsIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsCloudCostDataIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthUpdate;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::OpenAIUsageAndCostsIntegrationDataflowsRequest;

#[tokio::main]
async fn main() {
    let body =
        OpenAIUsageAndCostsIntegrationAccountUpdateRequest::new(
            OpenAIUsageAndCostsIntegrationAccountUpdateData::new(
                OpenAIUsageAndCostsIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        OpenAIUsageAndCostsIntegrationAccountAuthenticationUpdate
                        ::OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthUpdate(
                            Box::new(
                                OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthUpdate::new(
                                    OpenAIUsageAndCostsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                ).token("your-token".to_string()),
                            ),
                        ),
                    )
                    .dataflows(
                        OpenAIUsageAndCostsIntegrationDataflowsRequest
                        ::new().openai_cloud_cost_data(
                            OpenAIUsageAndCostsCloudCostDataIntegrationDataflowRequest::new().enabled(true),
                        ),
                    )
                    .name("openai-usage-and-costs-acme".to_string())
                    .settings(
                        OpenAIUsageAndCostsIntegrationAccountSettingsUpdate
                        ::new().custom_tags("team:ai,env:prod".to_string()),
                    ),
                "9c4e7a2b-1d5f-4e8a-b3c6-0f2d8a6e4b17".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.update_open_ai_usage_and_costs_integration_account",
        true,
    );
    let api = OpenAIUsageandCostsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_open_ai_usage_and_costs_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
