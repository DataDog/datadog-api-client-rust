// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Template variables saved views.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardTemplateVariablePreset {
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of variables.
    #[serde(rename = "template_variables")]
    pub template_variables:
        Option<Vec<crate::datadogV1::model::DashboardTemplateVariablePresetValue>>,
}

impl DashboardTemplateVariablePreset {
    pub fn new() -> DashboardTemplateVariablePreset {
        DashboardTemplateVariablePreset {
            name: None,
            template_variables: None,
        }
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn template_variables(
        &mut self,
        value: Vec<crate::datadogV1::model::DashboardTemplateVariablePresetValue>,
    ) -> &mut Self {
        self.template_variables = Some(value);
        self
    }
}

impl Default for DashboardTemplateVariablePreset {
    fn default() -> Self {
        Self::new()
    }
}
