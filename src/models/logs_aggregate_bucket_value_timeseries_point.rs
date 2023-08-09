/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsAggregateBucketValueTimeseriesPoint : A timeseries point



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogsAggregateBucketValueTimeseriesPoint {
    /// The time value for this point
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// The value for this point
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl LogsAggregateBucketValueTimeseriesPoint {
    /// A timeseries point
    pub fn new() -> LogsAggregateBucketValueTimeseriesPoint {
        LogsAggregateBucketValueTimeseriesPoint {
            time: None,
            value: None,
        }
    }
}


