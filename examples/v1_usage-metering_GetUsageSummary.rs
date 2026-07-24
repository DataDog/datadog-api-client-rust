// Get usage across your account returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_usage_metering::GetUsageSummaryOptionalParams;
use datadog_api_client::datadogV1::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI as UsageMeteringAPIV2;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();

    // Step 1: call v2 GetUsageSummaryAvailableFields to learn which keys exist
    let api_v2 = UsageMeteringAPIV2::with_config(configuration.clone());
    let fields_resp = api_v2.get_usage_summary_available_fields().await;
    let (response_fields, date_fields, date_org_fields) = match fields_resp {
        Ok(body) => {
            let attrs = body
                .data
                .and_then(|d| d.attributes)
                .unwrap_or_default();
            (
                attrs.response_fields.unwrap_or_default(),
                attrs.date_fields.unwrap_or_default(),
                attrs.date_org_fields.unwrap_or_default(),
            )
        }
        Err(e) => {
            println!("{:#?}", e);
            return;
        }
    };

    // Step 2: call v1 GetUsageSummary with include_org_details
    let api_v1 = UsageMeteringAPI::with_config(configuration);
    let resp = api_v1
        .get_usage_summary(
            DateTime::parse_from_rfc3339("2021-11-11T11:11:11.111000+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            GetUsageSummaryOptionalParams::default().include_org_details(true),
        )
        .await;

    let summary = match resp {
        Ok(value) => value,
        Err(e) => {
            println!("{:#?}", e);
            return;
        }
    };

    // Layer 1: iterate response_fields over top-level UsageSummaryResponse additional_properties
    println!("=== response-level additional_properties ===");
    for field in &response_fields {
        if let Some(val) = summary.additional_properties.get(field) {
            println!("  {field}: {val}");
        }
    }

    // Layer 2 & 3: iterate date_fields and date_org_fields over each UsageSummaryDate (and its orgs)
    if let Some(usage_dates) = &summary.usage {
        for usage_date in usage_dates {
            println!(
                "\n=== date {} additional_properties ===",
                usage_date
                    .date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "(unknown)".to_string())
            );
            for field in &date_fields {
                if let Some(val) = usage_date.additional_properties.get(field) {
                    println!("  {field}: {val}");
                }
            }

            if let Some(orgs) = &usage_date.orgs {
                for org in orgs {
                    println!(
                        "\n  === org {} additional_properties ===",
                        org.name
                            .as_deref()
                            .unwrap_or("(unknown)")
                    );
                    for field in &date_org_fields {
                        if let Some(val) = org.additional_properties.get(field) {
                            println!("    {field}: {val}");
                        }
                    }
                }
            }
        }
    }
}
