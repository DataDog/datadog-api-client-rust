/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsArchiveState : The state of the archive.

/// The state of the archive.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsArchiveState {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "WORKING")]
    WORKING,
    #[serde(rename = "FAILING")]
    FAILING,
    #[serde(rename = "WORKING_AUTH_LEGACY")]
    WORKING_AUTH_LEGACY,

}

impl ToString for LogsArchiveState {
    fn to_string(&self) -> String {
        match self {
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::WORKING => String::from("WORKING"),
            Self::FAILING => String::from("FAILING"),
            Self::WORKING_AUTH_LEGACY => String::from("WORKING_AUTH_LEGACY"),
        }
    }
}

impl Default for LogsArchiveState {
    fn default() -> LogsArchiveState {
        Self::UNKNOWN
    }
}




