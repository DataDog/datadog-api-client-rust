// Get available fields for usage summary returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api.get_usage_summary_available_fields().await;
    match resp {
        Ok(value) => {
            if let Some(data) = value.data {
                if let Some(attrs) = data.attributes {
                    let response_fields = attrs.response_fields.unwrap_or_default();
                    println!("response_fields ({}):", response_fields.len());
                    for f in &response_fields {
                        println!("  {f}");
                    }

                    let date_fields = attrs.date_fields.unwrap_or_default();
                    println!("date_fields ({}):", date_fields.len());
                    for f in &date_fields {
                        println!("  {f}");
                    }

                    let date_org_fields = attrs.date_org_fields.unwrap_or_default();
                    println!("date_org_fields ({}):", date_org_fields.len());
                    for f in &date_org_fields {
                        println!("  {f}");
                    }
                }
            }
        }
        Err(e) => println!("{:#?}", e),
    }
}
