// Update the MCP Cross-App Access issuer URL returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_organizations::OrganizationsAPI;
use datadog_api_client::datadogV2::model::McpCrossAppAccessIssuerUrlType;
use datadog_api_client::datadogV2::model::McpCrossAppAccessIssuerUrlUpdateAttributes;
use datadog_api_client::datadogV2::model::McpCrossAppAccessIssuerUrlUpdateData;
use datadog_api_client::datadogV2::model::McpCrossAppAccessIssuerUrlUpdateRequest;

#[tokio::main]
async fn main() {
    let body =
        McpCrossAppAccessIssuerUrlUpdateRequest::new(McpCrossAppAccessIssuerUrlUpdateData::new(
            McpCrossAppAccessIssuerUrlUpdateAttributes::new(
                "https://your-subdomain.okta.com".to_string(),
            ),
            McpCrossAppAccessIssuerUrlType::ORG_CONFIG,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.UpdateLoginOrgConfigsMcpCrossAppAccessIssuerUrl", true);
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api
        .update_login_org_configs_mcp_cross_app_access_issuer_url(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
