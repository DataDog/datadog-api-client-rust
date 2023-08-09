/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ScalarFormulaResponseType : The type of the resource. The value should always be scalar_response.

/// The type of the resource. The value should always be scalar_response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScalarFormulaResponseType {
    #[serde(rename = "scalar_response")]
    SCALAR_RESPONSE,

}

impl ToString for ScalarFormulaResponseType {
    fn to_string(&self) -> String {
        match self {
            Self::SCALAR_RESPONSE => String::from("scalar_response"),
        }
    }
}

impl Default for ScalarFormulaResponseType {
    fn default() -> ScalarFormulaResponseType {
        Self::SCALAR_RESPONSE
    }
}




