// Update tenancy config returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_oci_integration::OCIIntegrationAPI;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigData;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigDataAttributes;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigDataAttributesAuthCredentials;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigDataAttributesLogsConfig;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigDataAttributesMetricsConfig;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigDataAttributesRegionsConfig;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigDataType;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigRequest;

#[tokio::main]
async fn main() {
    let body = UpdateTenancyConfigRequest::new(
        UpdateTenancyConfigData::new(
            "ocid.tenancy.test".to_string(),
            UpdateTenancyConfigDataType::OCI_TENANCY,
        )
        .attributes(
            UpdateTenancyConfigDataAttributes::new()
                .auth_credentials(
                    UpdateTenancyConfigDataAttributesAuthCredentials::new(
                        r#"----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCdvSMmlfLyeD4M
QsA3WlrWBqKdWa5eVV3/uODyqT3wWMEMIJHcG3/quNs8nh9xrK1/JkQT2qoKEHqR
C5k59jN6Vp8em8ARJthMgam9K37ELt+IQ/G8ySTSuqZG8T4cHp/cs3fAclNqttOl
YnGr4RbVAgMBAAECggEAGZNLGbyCUbIRTW6Kh4d8ZVC+eZtJMqGmGJ3KfVaW8Pjn
QGWfSuJCEe2o2Y8G3phlidFauICnZ44enXA17Rhi+I/whnr7FIyQk2bR7rv+1Uhc
mOJygWX5eFFMsledgVAdIAl9Luk2nykx7Un3g6rtbl/Vs+5k4m7ITLFMpCHzsJLU
nm8kBzDOqY2JUkMd08nL88KL6QywWtal05UESzQpNFXd0e5kxYfexeMCsLsWP0mc
quMLRbn7NuBjCbe9VU2kmIvcfDDaWjurT7d5m1OXx1cc8p6P4PFZTVyCjdhiWOr3
LQXZ4/vdZNR3zgEHypRoM6D9Yq99LWUOUEMrdiSLQQKBgQDQkh7C1OtAXnpy7F6R
W+/I3zBHici2p7A57UT7VECQ1IVGg37/uus83DkuOtdZ33JmHLAVrwLFJvUlbyjx
l6dc/1ms40L5HFdLgaVtd4k0rSPFeOSDr6evz0lX4yBuzlP0fEh+o3XHW7mwe2G+
rWCULF/Uqza66fjbCSKMNgLIXQKBgQDBm9nZg/s4S0THWCFNWcB1tXBG0p/sH5eY
PC1H/VmTEINIixStrS4ufczf31X8rcoSjSbO7+vZDTTATdk7OLn1I2uGFVYl8M59
86BYT2Hi7cwp7YVzOc/cJigVeBAqSRW/iYYyWBEUTiW1gbkV0sRWwhPp67m+c0sP
XpY/iEZA2QKBgB1w8tynt4l/jKNaUEMOijt9ndALWATIiOy0XG9pxi9rgGCiwTOS
DBCsOXoYHjv2eayGUijNaoOv6xzcoxfvQ1WySdNIxTRq1ru20kYwgHKqGgmO9hrM
mcwMY5r/WZ2qjFlPjeAqbL62aPDLidGjoaVo2iIoBPK/gjxQ/5f0MS4N/YQ0zWoYBueSQ0DGs
-----END PRIVATE KEY-----"#
                            .to_string(),
                    )
                    .fingerprint("".to_string()),
                )
                .cost_collection_enabled(true)
                .home_region("us-ashburn-1".to_string())
                .logs_config(
                    UpdateTenancyConfigDataAttributesLogsConfig::new()
                        .compartment_tag_filters(vec![
                            "datadog:true".to_string(),
                            "env:prod".to_string(),
                        ])
                        .enabled(true)
                        .enabled_services(vec!["service_1".to_string(), "service_1".to_string()]),
                )
                .metrics_config(
                    UpdateTenancyConfigDataAttributesMetricsConfig::new()
                        .compartment_tag_filters(vec![
                            "datadog:true".to_string(),
                            "env:prod".to_string(),
                        ])
                        .enabled(true)
                        .excluded_services(vec!["service_1".to_string(), "service_1".to_string()]),
                )
                .regions_config(
                    UpdateTenancyConfigDataAttributesRegionsConfig::new()
                        .available(vec!["us-ashburn-1".to_string(), "us-phoenix-1".to_string()])
                        .disabled(vec!["us-phoenix-1".to_string()])
                        .enabled(vec!["us-ashburn-1".to_string()]),
                )
                .resource_collection_enabled(true)
                .user_ocid("ocid.user.test".to_string()),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = OCIIntegrationAPI::with_config(configuration);
    let resp = api
        .update_tenancy_config("tenancy_ocid".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
