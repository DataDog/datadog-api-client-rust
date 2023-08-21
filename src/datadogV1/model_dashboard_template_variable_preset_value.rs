// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardTemplateVariablePresetValue {
    /// The name of the variable.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// (deprecated) The value of the template variable within the saved view. Cannot be used in conjunction with `values`.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: String,
    /// One or many template variable values within the saved view, which will be unioned together using `OR` if more than one is specified. Cannot be used in conjunction with `value`.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Vec<String>,
}

