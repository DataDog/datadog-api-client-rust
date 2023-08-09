/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CloudConfigurationRuleCreatePayload : Create a new cloud configuration rule.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudConfigurationRuleCreatePayload {
    /// Description of generated findings and signals (severity and channels to be notified in case of a signal). Must contain exactly one item. 
    #[serde(rename = "cases")]
    pub cases: Vec<crate::models::CloudConfigurationRuleCaseCreate>,
    #[serde(rename = "complianceSignalOptions")]
    pub compliance_signal_options: Box<crate::models::CloudConfigurationRuleComplianceSignalOptions>,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::models::SecurityMonitoringFilter>>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    /// Message in markdown format for generated findings and signals.
    #[serde(rename = "message")]
    pub message: String,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "options")]
    pub options: Box<crate::models::CloudConfigurationRuleOptions>,
    /// Tags for generated findings and signals.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::CloudConfigurationRuleType>,
}

impl CloudConfigurationRuleCreatePayload {
    /// Create a new cloud configuration rule.
    pub fn new(cases: Vec<crate::models::CloudConfigurationRuleCaseCreate>, compliance_signal_options: crate::models::CloudConfigurationRuleComplianceSignalOptions, is_enabled: bool, message: String, name: String, options: crate::models::CloudConfigurationRuleOptions) -> CloudConfigurationRuleCreatePayload {
        CloudConfigurationRuleCreatePayload {
            cases,
            compliance_signal_options: Box::new(compliance_signal_options),
            filters: None,
            is_enabled,
            message,
            name,
            options: Box::new(options),
            tags: None,
            r#type: None,
        }
    }
}


