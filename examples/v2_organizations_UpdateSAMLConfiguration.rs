// Update a SAML configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_organizations::OrganizationsAPI;
use datadog_api_client::datadogV2::model::RelationshipToRoleData;
use datadog_api_client::datadogV2::model::RelationshipToRoles;
use datadog_api_client::datadogV2::model::RolesType;
use datadog_api_client::datadogV2::model::SAMLConfigurationRelationships;
use datadog_api_client::datadogV2::model::SAMLConfigurationUpdateAttributes;
use datadog_api_client::datadogV2::model::SAMLConfigurationUpdateData;
use datadog_api_client::datadogV2::model::SAMLConfigurationUpdateRequest;
use datadog_api_client::datadogV2::model::SAMLConfigurationsType;

#[tokio::main]
async fn main() {
    let body = SAMLConfigurationUpdateRequest::new(
        SAMLConfigurationUpdateData::new(
            "3653d3c6-0c75-11ea-ad28-fb5701eabc7d".to_string(),
            SAMLConfigurationsType::SAML_CONFIGURATIONS,
        )
        .attributes(
            SAMLConfigurationUpdateAttributes::new()
                .idp_initiated(true)
                .jit_domains(vec!["example.com".to_string()]),
        )
        .relationships(SAMLConfigurationRelationships::new().default_roles(
            RelationshipToRoles::new().data(vec![
                                RelationshipToRoleData::new()
                                    .id("3653d3c6-0c75-11ea-ad28-fb5701eabc7d".to_string())
                                    .type_(RolesType::ROLES)
                            ]),
        )),
    );
    let configuration = datadog::Configuration::new();
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api
        .update_saml_configuration("3653d3c6-0c75-11ea-ad28-fb5701eabc7d".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
