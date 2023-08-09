/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DowntimeStatus : The current status of the downtime.

/// The current status of the downtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DowntimeStatus {
    #[serde(rename = "active")]
    ACTIVE,
    #[serde(rename = "canceled")]
    CANCELED,
    #[serde(rename = "ended")]
    ENDED,
    #[serde(rename = "scheduled")]
    SCHEDULED,

}

impl ToString for DowntimeStatus {
    fn to_string(&self) -> String {
        match self {
            Self::ACTIVE => String::from("active"),
            Self::CANCELED => String::from("canceled"),
            Self::ENDED => String::from("ended"),
            Self::SCHEDULED => String::from("scheduled"),
        }
    }
}

impl Default for DowntimeStatus {
    fn default() -> DowntimeStatus {
        Self::ACTIVE
    }
}




