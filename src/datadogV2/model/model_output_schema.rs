// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A list of output parameters for the workflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutputSchema {
    /// The `OutputSchema` `parameters`.
    #[serde(rename = "parameters")]
    pub parameters: Option<Vec<crate::datadogV2::model::OutputSchemaParameters>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutputSchema {
    pub fn new() -> OutputSchema {
        OutputSchema {
            parameters: None,
            _unparsed: false,
        }
    }

    pub fn parameters(
        mut self,
        value: Vec<crate::datadogV2::model::OutputSchemaParameters>,
    ) -> Self {
        self.parameters = Some(value);
        self
    }
}

impl Default for OutputSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OutputSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutputSchemaVisitor;
        impl<'a> Visitor<'a> for OutputSchemaVisitor {
            type Value = OutputSchema;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut parameters: Option<Vec<crate::datadogV2::model::OutputSchemaParameters>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "parameters" => {
                            if v.is_null() {
                                continue;
                            }
                            parameters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = OutputSchema {
                    parameters,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutputSchemaVisitor)
    }
}
