// Unassign seats from users for a product code returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_seats::SeatsAPI;
use datadog_api_client::datadogV2::model::SeatAssignmentsDataType;
use datadog_api_client::datadogV2::model::UnassignSeatsUserRequest;
use datadog_api_client::datadogV2::model::UnassignSeatsUserRequestData;
use datadog_api_client::datadogV2::model::UnassignSeatsUserRequestDataAttributes;

#[tokio::main]
async fn main() {
    let body = UnassignSeatsUserRequest::new().data(
        UnassignSeatsUserRequestData::new(SeatAssignmentsDataType::SEAT_ASSIGNMENTS)
            .attributes(UnassignSeatsUserRequestDataAttributes::new().user_uuids(vec![])),
    );
    let configuration = datadog::Configuration::new();
    let api = SeatsAPI::with_config(configuration);
    let resp = api.unassign_seats_user_v2(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
