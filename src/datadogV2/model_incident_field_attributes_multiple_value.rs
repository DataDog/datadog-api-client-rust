// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentFieldAttributesMultipleValue {
    /// Type of the multiple value field definitions.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: IncidentFieldAttributesValueType,
    /// The multiple values selected for this field.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub value: datadog.NullableList[String],
}

