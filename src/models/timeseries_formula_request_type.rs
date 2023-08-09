/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// TimeseriesFormulaRequestType : The type of the resource. The value should always be timeseries_request.

/// The type of the resource. The value should always be timeseries_request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeseriesFormulaRequestType {
    #[serde(rename = "timeseries_request")]
    TIMESERIES_REQUEST,

}

impl ToString for TimeseriesFormulaRequestType {
    fn to_string(&self) -> String {
        match self {
            Self::TIMESERIES_REQUEST => String::from("timeseries_request"),
        }
    }
}

impl Default for TimeseriesFormulaRequestType {
    fn default() -> TimeseriesFormulaRequestType {
        Self::TIMESERIES_REQUEST
    }
}




