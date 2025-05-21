// Update tenancy config returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_oci_integration::OCIIntegrationAPI;
use datadog_api_client::datadogV2::model::OCIMetricsConfig;
use datadog_api_client::datadogV2::model::UpdateTenancyConfig;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigData;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigDataAttributes;
use datadog_api_client::datadogV2::model::UpdateTenancyConfigDataType;

#[tokio::main]
async fn main() {
    // there is a valid "oci_tenancy" resource in the system
    let oci_tenancy_data_id = std::env::var("OCI_TENANCY_DATA_ID").unwrap();
    let body = UpdateTenancyConfig::new().data(
        UpdateTenancyConfigData::new(
            oci_tenancy_data_id.clone(),
            UpdateTenancyConfigDataType::OCI_TENANCY,
        )
        .attributes(
            UpdateTenancyConfigDataAttributes::new()
                .home_region("us-sanjose-1".to_string())
                .metrics_config(
                    OCIMetricsConfig::new()
                        .compartment_tag_filters(vec![
                            "datadog:true".to_string(),
                            "env:prod".to_string(),
                        ])
                        .enabled(false)
                        .excluded_services(vec![]),
                )
                .resource_collection_enabled(false)
                .user_ocid("ocid1.user.test_updated".to_string()),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = OCIIntegrationAPI::with_config(configuration);
    let resp = api
        .update_tenancy_config(oci_tenancy_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
