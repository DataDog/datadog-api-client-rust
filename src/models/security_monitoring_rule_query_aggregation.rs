/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityMonitoringRuleQueryAggregation : The aggregation type.

/// The aggregation type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleQueryAggregation {
    #[serde(rename = "count")]
    COUNT,
    #[serde(rename = "cardinality")]
    CARDINALITY,
    #[serde(rename = "sum")]
    SUM,
    #[serde(rename = "max")]
    MAX,
    #[serde(rename = "new_value")]
    NEW_VALUE,
    #[serde(rename = "geo_data")]
    GEO_DATA,
    #[serde(rename = "event_count")]
    EVENT_COUNT,
    #[serde(rename = "none")]
    NONE,

}

impl ToString for SecurityMonitoringRuleQueryAggregation {
    fn to_string(&self) -> String {
        match self {
            Self::COUNT => String::from("count"),
            Self::CARDINALITY => String::from("cardinality"),
            Self::SUM => String::from("sum"),
            Self::MAX => String::from("max"),
            Self::NEW_VALUE => String::from("new_value"),
            Self::GEO_DATA => String::from("geo_data"),
            Self::EVENT_COUNT => String::from("event_count"),
            Self::NONE => String::from("none"),
        }
    }
}

impl Default for SecurityMonitoringRuleQueryAggregation {
    fn default() -> SecurityMonitoringRuleQueryAggregation {
        Self::COUNT
    }
}




