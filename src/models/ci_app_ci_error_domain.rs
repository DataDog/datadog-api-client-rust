/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CiAppCiErrorDomain : Error category used to differentiate between issues related to the developer or provider environments.

/// Error category used to differentiate between issues related to the developer or provider environments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CiAppCiErrorDomain {
    #[serde(rename = "provider")]
    PROVIDER,
    #[serde(rename = "user")]
    USER,
    #[serde(rename = "unknown")]
    UNKNOWN,

}

impl ToString for CiAppCiErrorDomain {
    fn to_string(&self) -> String {
        match self {
            Self::PROVIDER => String::from("provider"),
            Self::USER => String::from("user"),
            Self::UNKNOWN => String::from("unknown"),
        }
    }
}

impl Default for CiAppCiErrorDomain {
    fn default() -> CiAppCiErrorDomain {
        Self::PROVIDER
    }
}




