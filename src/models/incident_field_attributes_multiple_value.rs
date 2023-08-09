/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentFieldAttributesMultipleValue : A field with potentially multiple values selected.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentFieldAttributesMultipleValue {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::IncidentFieldAttributesValueType>,
    /// The multiple values selected for this field.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<Vec<String>>>,
}

impl IncidentFieldAttributesMultipleValue {
    /// A field with potentially multiple values selected.
    pub fn new() -> IncidentFieldAttributesMultipleValue {
        IncidentFieldAttributesMultipleValue {
            r#type: None,
            value: None,
        }
    }
}


