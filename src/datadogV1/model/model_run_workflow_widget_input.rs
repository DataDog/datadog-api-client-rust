// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to map a dashboard template variable to a workflow input.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RunWorkflowWidgetInput {
    /// Name of the workflow input.
    #[serde(rename = "name")]
    pub name: String,
    /// Dashboard template variable. Can be suffixed with '.value' or '.key'.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RunWorkflowWidgetInput {
    pub fn new(name: String, value: String) -> RunWorkflowWidgetInput {
        RunWorkflowWidgetInput {
            name,
            value,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for RunWorkflowWidgetInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RunWorkflowWidgetInputVisitor;
        impl<'a> Visitor<'a> for RunWorkflowWidgetInputVisitor {
            type Value = RunWorkflowWidgetInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut value: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = RunWorkflowWidgetInput {
                    name,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RunWorkflowWidgetInputVisitor)
    }
}
