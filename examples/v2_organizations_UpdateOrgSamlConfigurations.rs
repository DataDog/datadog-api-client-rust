// Update organization SAML preferences returns "No Content - The SAML preferences
// were successfully updated." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_organizations::OrganizationsAPI;
use datadog_api_client::datadogV2::model::SamlConfigurationsUpdateAttributes;
use datadog_api_client::datadogV2::model::SamlConfigurationsUpdateRequest;
use datadog_api_client::datadogV2::model::SamlConfigurationsUpdateRequestData;
use datadog_api_client::datadogV2::model::SamlConfigurationsUpdateRequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = SamlConfigurationsUpdateRequest::new(SamlConfigurationsUpdateRequestData::new(
        SamlConfigurationsUpdateAttributes::new(
            vec![Uuid::parse_str("19fcc38b-b651-46a0-b463-1f8f56c6a862").expect("invalid UUID")],
            vec!["example1.com".to_string(), "example2.com".to_string()],
        ),
        SamlConfigurationsUpdateRequestDataType::SAML_PREFERENCES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateOrgSamlConfigurations", true);
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api.update_org_saml_configurations(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
