// Update connection returns "Connection updated successfully" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;
use datadog_api_client::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems;
use datadog_api_client::datadogV2::model::UpdateConnectionRequest;
use datadog_api_client::datadogV2::model::UpdateConnectionRequestData;
use datadog_api_client::datadogV2::model::UpdateConnectionRequestDataAttributes;
use datadog_api_client::datadogV2::model::UpdateConnectionRequestDataAttributesFieldsToUpdateItems;
use datadog_api_client::datadogV2::model::UpdateConnectionRequestDataType;

#[tokio::main]
async fn main() {
    let body = UpdateConnectionRequest::new().data(
        UpdateConnectionRequestData::new(
            "crm-integration".to_string(),
            UpdateConnectionRequestDataType::CONNECTION_ID,
        )
        .attributes(
            UpdateConnectionRequestDataAttributes::new()
                .fields_to_add(vec![CreateConnectionRequestDataAttributesFieldsItems::new(
                    "nps_score".to_string(),
                    "net_promoter_score".to_string(),
                    "number".to_string(),
                )
                .description("Net Promoter Score from customer surveys".to_string())
                .display_name("NPS Score".to_string())
                .groups(vec!["Satisfaction".to_string(), "Metrics".to_string()])])
                .fields_to_delete(vec!["old_revenue_field".to_string()])
                .fields_to_update(vec![
                    UpdateConnectionRequestDataAttributesFieldsToUpdateItems::new(
                        "lifetime_value".to_string(),
                    )
                    .updated_display_name(r#"Customer Lifetime Value (`USD`)"#.to_string())
                    .updated_groups(vec!["Financial".to_string(), "Metrics".to_string()]),
                ]),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateConnection", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api.update_connection("users".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
