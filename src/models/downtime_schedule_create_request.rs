/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DowntimeScheduleCreateRequest : Schedule for the downtime.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DowntimeScheduleCreateRequest {
    /// A list of downtime recurrences.
    #[serde(rename = "recurrences")]
    pub recurrences: Vec<crate::models::DowntimeScheduleRecurrenceCreateUpdateRequest>,
    /// The timezone in which to schedule the downtime.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// ISO-8601 Datetime to end the downtime. Must include a UTC offset of zero. If not provided, the downtime continues forever.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end: Option<Option<String>>,
    /// ISO-8601 Datetime to start the downtime. Must include a UTC offset of zero. If not provided, the downtime starts the moment it is created.
    #[serde(rename = "start", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start: Option<Option<String>>,
}

impl DowntimeScheduleCreateRequest {
    /// Schedule for the downtime.
    pub fn new(recurrences: Vec<crate::models::DowntimeScheduleRecurrenceCreateUpdateRequest>) -> DowntimeScheduleCreateRequest {
        DowntimeScheduleCreateRequest {
            recurrences,
            timezone: None,
            end: None,
            start: None,
        }
    }
}


