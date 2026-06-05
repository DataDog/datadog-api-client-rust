// Update organization SAML preferences returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_organizations::OrganizationsAPI;
use datadog_api_client::datadogV2::model::OrgSAMLPreferencesAttributes;
use datadog_api_client::datadogV2::model::OrgSAMLPreferencesData;
use datadog_api_client::datadogV2::model::OrgSAMLPreferencesType;
use datadog_api_client::datadogV2::model::OrgSAMLPreferencesUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body =
        OrgSAMLPreferencesUpdateRequest::new(
            OrgSAMLPreferencesData::new(
                OrgSAMLPreferencesAttributes::new(
                    vec![Uuid::parse_str("8dd1cf3c-0c75-11ea-ad28-fb5701eabc7d")
                        .expect("invalid UUID")],
                    vec!["example.com".to_string()],
                ),
                OrgSAMLPreferencesType::SAML_PREFERENCES,
            )
            .id("00000000-0000-0000-0000-000000000000".to_string()),
        );
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
