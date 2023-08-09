/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CostByOrgType : Type of cost data.

/// Type of cost data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CostByOrgType {
    #[serde(rename = "cost_by_org")]
    COST_BY_ORG,

}

impl ToString for CostByOrgType {
    fn to_string(&self) -> String {
        match self {
            Self::COST_BY_ORG => String::from("cost_by_org"),
        }
    }
}

impl Default for CostByOrgType {
    fn default() -> CostByOrgType {
        Self::COST_BY_ORG
    }
}




