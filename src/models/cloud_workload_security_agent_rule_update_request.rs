/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CloudWorkloadSecurityAgentRuleUpdateRequest : Request object that includes the Agent rule with the attributes to update.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleUpdateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::CloudWorkloadSecurityAgentRuleUpdateData>,
}

impl CloudWorkloadSecurityAgentRuleUpdateRequest {
    /// Request object that includes the Agent rule with the attributes to update.
    pub fn new(data: crate::models::CloudWorkloadSecurityAgentRuleUpdateData) -> CloudWorkloadSecurityAgentRuleUpdateRequest {
        CloudWorkloadSecurityAgentRuleUpdateRequest {
            data: Box::new(data),
        }
    }
}


