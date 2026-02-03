// Create tenancy config returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_oci_integration::OCIIntegrationAPI;
use datadog_api_client::datadogV2::model::CreateTenancyConfigData;
use datadog_api_client::datadogV2::model::CreateTenancyConfigDataAttributes;
use datadog_api_client::datadogV2::model::CreateTenancyConfigDataAttributesAuthCredentials;
use datadog_api_client::datadogV2::model::CreateTenancyConfigDataAttributesLogsConfig;
use datadog_api_client::datadogV2::model::CreateTenancyConfigDataAttributesMetricsConfig;
use datadog_api_client::datadogV2::model::CreateTenancyConfigDataAttributesRegionsConfig;
use datadog_api_client::datadogV2::model::CreateTenancyConfigRequest;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigDataType;

#[tokio::main]
async fn main() {
    let body = CreateTenancyConfigRequest::new(
        CreateTenancyConfigData::new(
            "ocid.tenancy.test".to_string(),
            UpdateTenancyConfigDataType::OCI_TENANCY,
        )
        .attributes(
            CreateTenancyConfigDataAttributes::new(
                CreateTenancyConfigDataAttributesAuthCredentials::new(
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
                "us-ashburn-1".to_string(),
                "ocid.user.test".to_string(),
            )
            .config_version(None)
            .cost_collection_enabled(true)
            .dd_compartment_id("ocid.compartment.test".to_string())
            .dd_stack_id("ocid.stack.test".to_string())
            .logs_config(
                CreateTenancyConfigDataAttributesLogsConfig::new()
                    .compartment_tag_filters(vec![
                        "datadog:true".to_string(),
                        "env:prod".to_string(),
                    ])
                    .enabled(true)
                    .enabled_services(vec!["service_1".to_string(), "service_1".to_string()]),
            )
            .metrics_config(
                CreateTenancyConfigDataAttributesMetricsConfig::new()
                    .compartment_tag_filters(vec![
                        "datadog:true".to_string(),
                        "env:prod".to_string(),
                    ])
                    .enabled(true)
                    .excluded_services(vec!["service_1".to_string(), "service_1".to_string()]),
            )
            .regions_config(
                CreateTenancyConfigDataAttributesRegionsConfig::new()
                    .available(vec!["us-ashburn-1".to_string(), "us-phoenix-1".to_string()])
                    .disabled(vec!["us-phoenix-1".to_string()])
                    .enabled(vec!["us-ashburn-1".to_string()]),
            )
            .resource_collection_enabled(true),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTenancyConfig", true);
    let api = OCIIntegrationAPI::with_config(configuration);
    let resp = api.create_tenancy_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
