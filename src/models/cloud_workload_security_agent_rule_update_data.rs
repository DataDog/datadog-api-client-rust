/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CloudWorkloadSecurityAgentRuleUpdateData : Object for a single Agent rule.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleUpdateData {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::CloudWorkloadSecurityAgentRuleUpdateAttributes>,
    #[serde(rename = "type")]
    pub r#type: crate::models::CloudWorkloadSecurityAgentRuleType,
}

impl CloudWorkloadSecurityAgentRuleUpdateData {
    /// Object for a single Agent rule.
    pub fn new(attributes: crate::models::CloudWorkloadSecurityAgentRuleUpdateAttributes, r#type: crate::models::CloudWorkloadSecurityAgentRuleType) -> CloudWorkloadSecurityAgentRuleUpdateData {
        CloudWorkloadSecurityAgentRuleUpdateData {
            attributes: Box::new(attributes),
            r#type,
        }
    }
}


