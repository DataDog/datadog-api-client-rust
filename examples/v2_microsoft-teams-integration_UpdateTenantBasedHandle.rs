// Update tenant-based handle returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleType;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateTenantBasedHandleRequest;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateTenantBasedHandleRequestData;

#[tokio::main]
async fn main() {
    let body = MicrosoftTeamsUpdateTenantBasedHandleRequest::new(
        MicrosoftTeamsUpdateTenantBasedHandleRequestData::new(
            MicrosoftTeamsTenantBasedHandleAttributes::new()
                .channel_id("fake-channel-id".to_string())
                .name("fake-handle-name".to_string())
                .team_id("00000000-0000-0000-0000-000000000000".to_string())
                .tenant_id("00000000-0000-0000-0000-000000000001".to_string()),
            MicrosoftTeamsTenantBasedHandleType::TENANT_BASED_HANDLE,
        ),
    );

    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_tenant_based_handle("handle_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
