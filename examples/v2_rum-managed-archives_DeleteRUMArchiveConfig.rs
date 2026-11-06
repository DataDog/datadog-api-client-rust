// Delete an archive configuration returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_managed_archives::RUMManagedArchivesAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMManagedArchivesAPI::with_config(configuration);
    let resp = api
        .delete_rum_archive_config(
            Uuid::parse_str("a33671aa-24fd-4dcd-ba4b-5bbdbafe7690").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
