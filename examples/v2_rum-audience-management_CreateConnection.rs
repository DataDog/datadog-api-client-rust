// Create connection returns "Connection created successfully" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;
use datadog_api_client::datadogV2::model::CreateConnectionRequest;
use datadog_api_client::datadogV2::model::CreateConnectionRequestData;
use datadog_api_client::datadogV2::model::CreateConnectionRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems;
use datadog_api_client::datadogV2::model::UpdateConnectionRequestDataType;

#[tokio::main]
async fn main() {
    let body = CreateConnectionRequest::new().data(
        CreateConnectionRequestData::new(UpdateConnectionRequestDataType::CONNECTION_ID)
            .attributes(
                CreateConnectionRequestDataAttributes::new(
                    "user_email".to_string(),
                    "email".to_string(),
                    "ref_table".to_string(),
                )
                .fields(vec![
                    CreateConnectionRequestDataAttributesFieldsItems::new(
                        "customer_tier".to_string(),
                        "subscription_tier".to_string(),
                        "string".to_string(),
                    )
                    .description(r#"Customer subscription tier from `CRM`"#.to_string())
                    .display_name("Customer Tier".to_string()),
                    CreateConnectionRequestDataAttributesFieldsItems::new(
                        "lifetime_value".to_string(),
                        "ltv".to_string(),
                        "number".to_string(),
                    )
                    .description(r#"Customer lifetime value in `USD`"#.to_string())
                    .display_name("Lifetime Value".to_string()),
                ]),
            )
            .id("crm-integration".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateConnection", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api.create_connection("users".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
