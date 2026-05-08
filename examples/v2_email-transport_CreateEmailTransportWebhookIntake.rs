// Ingest email transport webhook events returns "No Content" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_email_transport::EmailTransportAPI;
use datadog_api_client::datadogV2::model::TransportWebhookLog;
use datadog_api_client::datadogV2::model::TransportWebhookLogAttributes;
use datadog_api_client::datadogV2::model::TransportWebhookLogEmail;
use datadog_api_client::datadogV2::model::TransportWebhookLogIpAttribute;
use datadog_api_client::datadogV2::model::TransportWebhookLogMessage;
use datadog_api_client::datadogV2::model::TransportWebhookLogMessageAuth;
use datadog_api_client::datadogV2::model::TransportWebhookLogMessageCustomArgs;
use datadog_api_client::datadogV2::model::TransportWebhookLogMessageId;
use datadog_api_client::datadogV2::model::TransportWebhookLogMessageResponse;
use datadog_api_client::datadogV2::model::TransportWebhookLogMessageTimestamp;
use datadog_api_client::datadogV2::model::TransportWebhookLogNetwork;
use datadog_api_client::datadogV2::model::TransportWebhookLogNetworkIp;
use datadog_api_client::datadogV2::model::TransportWebhookLogOrgMetadata;

#[tokio::main]
async fn main() {
    let body = vec![TransportWebhookLog::new(
        TransportWebhookLogAttributes::new()
            .category(vec!["transactional".to_string()])
            .email(
                TransportWebhookLogEmail::new()
                    .address("user@example.com".to_string())
                    .domain("example.com".to_string())
                    .subject("[Monitor Alert] CPU usage is high".to_string())
                    .type_(vec!["transactional".to_string()]),
            )
            .email_id("abc123-def456".to_string())
            .email_type_display_name("Monitor Alert".to_string())
            .message(
                TransportWebhookLogMessage::new()
                    .auth(
                        TransportWebhookLogMessageAuth::new()
                            .delivered_with_tls("TLSv1.2".to_string()),
                    )
                    .custom_args(
                        TransportWebhookLogMessageCustomArgs::new()
                            .email_id("abc123-def456".to_string())
                            .email_type_display_name("Monitor Alert".to_string())
                            .org_uuid("8dee7c38-00cb-11ea-a77b-8b5a08d3b091".to_string())
                            .queue_time("2024-01-15T10:29:00Z".to_string())
                            .subject("[Monitor Alert] CPU usage is high".to_string()),
                    )
                    .id(TransportWebhookLogMessageId::new()
                        .message_id("<message-id@example.com>".to_string())
                        .smtp_id("<abc123@mail.example.com>".to_string())
                        .transport_event_id("evt_abc123".to_string()))
                    .name("delivered".to_string())
                    .response(
                        TransportWebhookLogMessageResponse::new()
                            .enhanced_smtp_code("2.0.0".to_string())
                            .reason("250 2.0.0 OK".to_string())
                            .smtp_code("250".to_string()),
                    )
                    .sender_ip("192.168.1.1".to_string())
                    .timestamp(
                        TransportWebhookLogMessageTimestamp::new()
                            .event_timestamp(1705312200.0 as f64)
                            .lifetime(3.2 as f64)
                            .queue_time(1.5 as f64)
                            .scheduled_time(1705312190.0 as f64),
                    ),
            )
            .network(
                TransportWebhookLogNetwork::new().ip(TransportWebhookLogNetworkIp::new()
                    .attributes(vec![TransportWebhookLogIpAttribute::new()
                        .ip("192.168.1.1".to_string())
                        .source(vec!["sendgrid".to_string()])])
                    .list(vec!["192.168.1.1".to_string()])),
            )
            .org(1234)
            .org_metadata(TransportWebhookLogOrgMetadata::new())
            .org_uuid("8dee7c38-00cb-11ea-a77b-8b5a08d3b091".to_string())
            .queue_time("2024-01-15T10:29:00Z".to_string())
            .subject("[Monitor Alert] CPU usage is high".to_string())
            .useragent("Mozilla/5.0".to_string()),
        DateTime::parse_from_rfc3339("2024-01-15T10:30:00+00:00")
            .expect("Failed to parse datetime")
            .with_timezone(&Utc),
        "AQAAAZPHnBT0TwJAdgAAAABBWlBIblVlNEFBQ0dFMmVkYTFDSnRR".to_string(),
        "sendgrid".to_string(),
        "info".to_string(),
        vec!["env:production".to_string()],
    )];
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateEmailTransportWebhookIntake", true);
    let api = EmailTransportAPI::with_config(configuration);
    let resp = api.create_email_transport_webhook_intake(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
