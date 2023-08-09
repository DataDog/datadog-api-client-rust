/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ScalarFormulaRequestAttributes : The object describing a scalar formula request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalarFormulaRequestAttributes {
    /// List of formulas to be calculated and returned as responses.
    #[serde(rename = "formulas", skip_serializing_if = "Option::is_none")]
    pub formulas: Option<Vec<crate::models::QueryFormula>>,
    /// Start date (inclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "from")]
    pub from: i64,
    /// List of queries to be run and used as inputs to the formulas.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::models::ScalarQuery>,
    /// End date (exclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "to")]
    pub to: i64,
}

impl ScalarFormulaRequestAttributes {
    /// The object describing a scalar formula request.
    pub fn new(from: i64, queries: Vec<crate::models::ScalarQuery>, to: i64) -> ScalarFormulaRequestAttributes {
        ScalarFormulaRequestAttributes {
            formulas: None,
            from,
            queries,
            to,
        }
    }
}


