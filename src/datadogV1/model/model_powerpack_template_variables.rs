// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack template variables.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackTemplateVariables {
    /// Template variables controlled at the powerpack level.
    #[serde(rename = "controlled_by_powerpack")]
    pub controlled_by_powerpack:
        Option<Vec<crate::datadogV1::model::PowerpackTemplateVariableContents>>,
    /// Template variables controlled by the external resource, such as the dashboard this powerpack is on.
    #[serde(rename = "controlled_externally")]
    pub controlled_externally:
        Option<Vec<crate::datadogV1::model::PowerpackTemplateVariableContents>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackTemplateVariables {
    pub fn new() -> PowerpackTemplateVariables {
        PowerpackTemplateVariables {
            controlled_by_powerpack: None,
            controlled_externally: None,
            _unparsed: false,
        }
    }

    pub fn controlled_by_powerpack(
        &mut self,
        value: Vec<crate::datadogV1::model::PowerpackTemplateVariableContents>,
    ) -> &mut Self {
        self.controlled_by_powerpack = Some(value);
        self
    }

    pub fn controlled_externally(
        &mut self,
        value: Vec<crate::datadogV1::model::PowerpackTemplateVariableContents>,
    ) -> &mut Self {
        self.controlled_externally = Some(value);
        self
    }
}

impl Default for PowerpackTemplateVariables {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PowerpackTemplateVariables {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackTemplateVariablesVisitor;
        impl<'a> Visitor<'a> for PowerpackTemplateVariablesVisitor {
            type Value = PowerpackTemplateVariables;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut controlled_by_powerpack: Option<
                    Vec<crate::datadogV1::model::PowerpackTemplateVariableContents>,
                > = None;
                let mut controlled_externally: Option<
                    Vec<crate::datadogV1::model::PowerpackTemplateVariableContents>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "controlled_by_powerpack" => {
                            if v.is_null() {
                                continue;
                            }
                            controlled_by_powerpack =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "controlled_externally" => {
                            if v.is_null() {
                                continue;
                            }
                            controlled_externally =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = PowerpackTemplateVariables {
                    controlled_by_powerpack,
                    controlled_externally,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackTemplateVariablesVisitor)
    }
}
