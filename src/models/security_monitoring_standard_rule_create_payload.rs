/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityMonitoringStandardRuleCreatePayload : Create a new rule.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityMonitoringStandardRuleCreatePayload {
    /// Cases for generating signals.
    #[serde(rename = "cases")]
    pub cases: Vec<crate::models::SecurityMonitoringRuleCaseCreate>,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::models::SecurityMonitoringFilter>>,
    /// Whether the notifications include the triggering group-by values in their title.
    #[serde(rename = "hasExtendedTitle", skip_serializing_if = "Option::is_none")]
    pub has_extended_title: Option<bool>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    /// Message for generated signals.
    #[serde(rename = "message")]
    pub message: String,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "options")]
    pub options: Box<crate::models::SecurityMonitoringRuleOptions>,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::models::SecurityMonitoringStandardRuleQuery>,
    /// Tags for generated signals.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::SecurityMonitoringRuleTypeCreate>,
}

impl SecurityMonitoringStandardRuleCreatePayload {
    /// Create a new rule.
    pub fn new(cases: Vec<crate::models::SecurityMonitoringRuleCaseCreate>, is_enabled: bool, message: String, name: String, options: crate::models::SecurityMonitoringRuleOptions, queries: Vec<crate::models::SecurityMonitoringStandardRuleQuery>) -> SecurityMonitoringStandardRuleCreatePayload {
        SecurityMonitoringStandardRuleCreatePayload {
            cases,
            filters: None,
            has_extended_title: None,
            is_enabled,
            message,
            name,
            options: Box::new(options),
            queries,
            tags: None,
            r#type: None,
        }
    }
}


