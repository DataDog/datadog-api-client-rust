/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CloudWorkloadSecurityAgentRuleCreateRequest : Request object that includes the Agent rule to create.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleCreateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::CloudWorkloadSecurityAgentRuleCreateData>,
}

impl CloudWorkloadSecurityAgentRuleCreateRequest {
    /// Request object that includes the Agent rule to create.
    pub fn new(data: crate::models::CloudWorkloadSecurityAgentRuleCreateData) -> CloudWorkloadSecurityAgentRuleCreateRequest {
        CloudWorkloadSecurityAgentRuleCreateRequest {
            data: Box::new(data),
        }
    }
}


