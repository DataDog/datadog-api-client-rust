// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Template variables saved views.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardTemplateVariablePreset {
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of variables.
    #[serde(rename = "template_variables")]
    pub template_variables:
        Option<Vec<crate::datadogV1::model::DashboardTemplateVariablePresetValue>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardTemplateVariablePreset {
    pub fn new() -> DashboardTemplateVariablePreset {
        DashboardTemplateVariablePreset {
            name: None,
            template_variables: None,
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn template_variables(
        mut self,
        value: Vec<crate::datadogV1::model::DashboardTemplateVariablePresetValue>,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }
}

impl Default for DashboardTemplateVariablePreset {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardTemplateVariablePreset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardTemplateVariablePresetVisitor;
        impl<'a> Visitor<'a> for DashboardTemplateVariablePresetVisitor {
            type Value = DashboardTemplateVariablePreset;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut template_variables: Option<
                    Vec<crate::datadogV1::model::DashboardTemplateVariablePresetValue>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_variables" => {
                            if v.is_null() {
                                continue;
                            }
                            template_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DashboardTemplateVariablePreset {
                    name,
                    template_variables,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardTemplateVariablePresetVisitor)
    }
}
