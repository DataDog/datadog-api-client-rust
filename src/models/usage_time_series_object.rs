/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageTimeSeriesObject : Usage timeseries data.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UsageTimeSeriesObject {
    /// Datetime in ISO-8601 format, UTC. The hour for the usage.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Contains the number measured for the given usage_type during the hour.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<i64>>,
}

impl UsageTimeSeriesObject {
    /// Usage timeseries data.
    pub fn new() -> UsageTimeSeriesObject {
        UsageTimeSeriesObject {
            timestamp: None,
            value: None,
        }
    }
}


