// Create tenant-based handle returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsCreateTenantBasedHandleRequest;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleRequestAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleRequestData;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleType;

#[tokio::main]
async fn main() {
    let body = MicrosoftTeamsCreateTenantBasedHandleRequest::new(
        MicrosoftTeamsTenantBasedHandleRequestData::new(
            MicrosoftTeamsTenantBasedHandleRequestAttributes::new(
                "fake-channel-id".to_string(),
                "fake-handle-name".to_string(),
                "00000000-0000-0000-0000-000000000000".to_string(),
                "00000000-0000-0000-0000-000000000001".to_string(),
            ),
            MicrosoftTeamsTenantBasedHandleType::TENANT_BASED_HANDLE,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api.create_tenant_based_handle(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
