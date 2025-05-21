// Create tenancy config returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_oci_integration::OCIIntegrationAPI;
use datadog_api_client::datadogV2::model::AuthCredentials;
use datadog_api_client::datadogV2::model::CreateTenancyConfig;
use datadog_api_client::datadogV2::model::CreateTenancyConfigData;
use datadog_api_client::datadogV2::model::CreateTenancyConfigDataAttributes;
use datadog_api_client::datadogV2::model::CreateTenancyConfigDataType;
use datadog_api_client::datadogV2::model::OCILogsConfig;
use datadog_api_client::datadogV2::model::OCIMetricsConfig;

#[tokio::main]
async fn main() {
    let body = CreateTenancyConfig::new().data(
        CreateTenancyConfigData::new(
            "ocid1.tenancy.dummy_value".to_string(),
            CreateTenancyConfigDataType::OCI_TENANCY,
        )
        .attributes(
            CreateTenancyConfigDataAttributes::new(
                AuthCredentials::new(
                    "a7:b5:54:f2:da:a2:d7:b0:ed:f4:79:47:93:64:12:b1".to_string(),
                    r#"-----BEGIN PRIVATE KEY-----
o9kEwoumo8yHVn5Ztp4F2cxaD6+MzSJ/I6WesPyePUD7sPeorXByg1UNOXyzqDub
/aU4/sNo2f8epM9l7QGiCtY=
-----END PRIVATE KEY-----"#
                        .to_string(),
                ),
                "us-ashburn-1".to_string(),
                "ocid1.user.test".to_string(),
            )
            .config_version(2)
            .logs_config(
                OCILogsConfig::new()
                    .compartment_tag_filters(vec![
                        "datadog:true".to_string(),
                        "env:prod".to_string(),
                    ])
                    .enabled(true)
                    .enabled_services(vec!["oacnativeproduction".to_string()]),
            )
            .metrics_config(
                OCIMetricsConfig::new()
                    .compartment_tag_filters(vec![
                        "datadog:true".to_string(),
                        "env:prod".to_string(),
                    ])
                    .enabled(true)
                    .excluded_services(vec!["oci_compute".to_string()]),
            )
            .resource_collection_enabled(true),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = OCIIntegrationAPI::with_config(configuration);
    let resp = api.create_tenancy_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
