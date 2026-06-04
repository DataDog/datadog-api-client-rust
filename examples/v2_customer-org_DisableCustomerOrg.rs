// Disable the authenticated customer organization returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_customer_org::CustomerOrgAPI;
use datadog_api_client::datadogV2::model::CustomerOrgDisableRequest;
use datadog_api_client::datadogV2::model::CustomerOrgDisableRequestAttributes;
use datadog_api_client::datadogV2::model::CustomerOrgDisableRequestData;
use datadog_api_client::datadogV2::model::CustomerOrgDisableType;

#[tokio::main]
async fn main() {
    let body = CustomerOrgDisableRequest::new(
        CustomerOrgDisableRequestData::new(CustomerOrgDisableType::CUSTOMER_ORG_DISABLE)
            .attributes(
                CustomerOrgDisableRequestAttributes::new()
                    .org_uuid("abcdef01-2345-6789-abcd-ef0123456789".to_string()),
            )
            .id("1".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DisableCustomerOrg", true);
    let api = CustomerOrgAPI::with_config(configuration);
    let resp = api.disable_customer_org(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
