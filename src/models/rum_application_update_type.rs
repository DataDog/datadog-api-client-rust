/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RumApplicationUpdateType : RUM application update type.

/// RUM application update type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RumApplicationUpdateType {
    #[serde(rename = "rum_application_update")]
    RUM_APPLICATION_UPDATE,

}

impl ToString for RumApplicationUpdateType {
    fn to_string(&self) -> String {
        match self {
            Self::RUM_APPLICATION_UPDATE => String::from("rum_application_update"),
        }
    }
}

impl Default for RumApplicationUpdateType {
    fn default() -> RumApplicationUpdateType {
        Self::RUM_APPLICATION_UPDATE
    }
}




