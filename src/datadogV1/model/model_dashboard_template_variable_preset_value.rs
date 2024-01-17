// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Template variables saved views.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardTemplateVariablePresetValue {
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// (deprecated) The value of the template variable within the saved view. Cannot be used in conjunction with `values`.
    #[deprecated]
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// One or many template variable values within the saved view, which will be unioned together using `OR` if more than one is specified. Cannot be used in conjunction with `value`.
    #[serde(rename = "values")]
    pub values: Option<Vec<String>>,
}

impl DashboardTemplateVariablePresetValue {
    pub fn new() -> DashboardTemplateVariablePresetValue {
        #[allow(deprecated)]
        DashboardTemplateVariablePresetValue {
            name: None,
            value: None,
            values: None,
        }
    }
}
