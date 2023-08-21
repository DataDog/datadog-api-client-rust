// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectableTemplateVariableItems {
    /// The default value of the template variable.
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: String,
    /// Name of the template variable.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The tag/attribute key associated with the template variable.
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: String,
    /// List of visible tag values on the shared dashboard.
    #[serde(rename = "visible_tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub visible_tags: datadog.NullableList[String],
}

