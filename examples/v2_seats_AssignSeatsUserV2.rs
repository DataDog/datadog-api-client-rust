// Assign seats to users for a product code returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_seats::SeatsAPI;
use datadog_api_client::datadogV2::model::AssignSeatsUserRequest;
use datadog_api_client::datadogV2::model::AssignSeatsUserRequestData;
use datadog_api_client::datadogV2::model::AssignSeatsUserRequestDataAttributes;
use datadog_api_client::datadogV2::model::SeatAssignmentsDataType;

#[tokio::main]
async fn main() {
    let body = AssignSeatsUserRequest::new().data(
        AssignSeatsUserRequestData::new(SeatAssignmentsDataType::SEAT_ASSIGNMENTS)
            .attributes(AssignSeatsUserRequestDataAttributes::new().user_uuids(vec![])),
    );
    let configuration = datadog::Configuration::new();
    let api = SeatsAPI::with_config(configuration);
    let resp = api.assign_seats_user_v2(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
