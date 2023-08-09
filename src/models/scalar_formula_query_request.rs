/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ScalarFormulaQueryRequest : A wrapper request around one scalar query to be executed.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalarFormulaQueryRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::ScalarFormulaRequest>,
}

impl ScalarFormulaQueryRequest {
    /// A wrapper request around one scalar query to be executed.
    pub fn new(data: crate::models::ScalarFormulaRequest) -> ScalarFormulaQueryRequest {
        ScalarFormulaQueryRequest {
            data: Box::new(data),
        }
    }
}


