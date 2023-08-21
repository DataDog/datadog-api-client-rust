// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardTemplateVariable {
    /// The list of values that the template variable drop-down is limited to.
    #[serde(rename = "available_values", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub available_values: datadog.NullableList[String],
    /// (deprecated) The default value for the template variable on dashboard load. Cannot be used in conjunction with `defaults`.
    #[serde(rename = "default", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub default: Option<String>,
    /// One or many default values for template variables on load. If more than one default is specified, they will be unioned together with `OR`. Cannot be used in conjunction with `default`.
    #[serde(rename = "defaults", skip_serializing_if = "Option::is_none")]
    pub defaults: Vec<String>,
    /// The name of the variable.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The tag prefix associated with the variable. Only tags with this prefix appear in the variable drop-down.
    #[serde(rename = "prefix", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub prefix: Option<String>,
}

