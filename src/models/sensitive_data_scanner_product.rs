/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SensitiveDataScannerProduct : Datadog product onto which Sensitive Data Scanner can be activated.

/// Datadog product onto which Sensitive Data Scanner can be activated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SensitiveDataScannerProduct {
    #[serde(rename = "logs")]
    LOGS,
    #[serde(rename = "rum")]
    RUM,
    #[serde(rename = "events")]
    EVENTS,
    #[serde(rename = "apm")]
    APM,

}

impl ToString for SensitiveDataScannerProduct {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS => String::from("logs"),
            Self::RUM => String::from("rum"),
            Self::EVENTS => String::from("events"),
            Self::APM => String::from("apm"),
        }
    }
}

impl Default for SensitiveDataScannerProduct {
    fn default() -> SensitiveDataScannerProduct {
        Self::LOGS
    }
}




