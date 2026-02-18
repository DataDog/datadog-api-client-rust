// Unassign seats from users returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_seats::SeatsAPI;
use datadog_api_client::datadogV2::model::SeatAssignmentsDataType;
use datadog_api_client::datadogV2::model::UnassignSeatsUserRequest;
use datadog_api_client::datadogV2::model::UnassignSeatsUserRequestData;
use datadog_api_client::datadogV2::model::UnassignSeatsUserRequestDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let body = UnassignSeatsUserRequest::new().data(UnassignSeatsUserRequestData::new(
        UnassignSeatsUserRequestDataAttributes::new(
            "incident_response".to_string(),
            vec![user_data_id.clone()],
        ),
        SeatAssignmentsDataType::SEAT_ASSIGNMENTS,
    ));
    let configuration = datadog::Configuration::new();
    let api = SeatsAPI::with_config(configuration);
    let resp = api.unassign_seats_user(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
