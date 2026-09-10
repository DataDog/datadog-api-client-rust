// List On-Call schedules returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::ListOnCallSchedulesOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let response =
        api.list_on_call_schedules_with_pagination(ListOnCallSchedulesOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
