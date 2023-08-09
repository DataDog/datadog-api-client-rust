/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ScalarFormulaRequestType : The type of the resource. The value should always be scalar_request.

/// The type of the resource. The value should always be scalar_request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScalarFormulaRequestType {
    #[serde(rename = "scalar_request")]
    SCALAR_REQUEST,

}

impl ToString for ScalarFormulaRequestType {
    fn to_string(&self) -> String {
        match self {
            Self::SCALAR_REQUEST => String::from("scalar_request"),
        }
    }
}

impl Default for ScalarFormulaRequestType {
    fn default() -> ScalarFormulaRequestType {
        Self::SCALAR_REQUEST
    }
}




