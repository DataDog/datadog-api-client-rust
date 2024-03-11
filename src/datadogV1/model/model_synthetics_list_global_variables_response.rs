// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing an array of Synthetic global variables.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsListGlobalVariablesResponse {
    /// Array of Synthetic global variables.
    #[serde(rename = "variables")]
    pub variables: Option<Vec<crate::datadogV1::model::SyntheticsGlobalVariable>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsListGlobalVariablesResponse {
    pub fn new() -> SyntheticsListGlobalVariablesResponse {
        SyntheticsListGlobalVariablesResponse {
            variables: None,
            _unparsed: false,
        }
    }

    pub fn variables(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsGlobalVariable>,
    ) -> &mut Self {
        self.variables = Some(value);
        self
    }
}

impl Default for SyntheticsListGlobalVariablesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsListGlobalVariablesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsListGlobalVariablesResponseVisitor;
        impl<'a> Visitor<'a> for SyntheticsListGlobalVariablesResponseVisitor {
            type Value = SyntheticsListGlobalVariablesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut variables: Option<Vec<crate::datadogV1::model::SyntheticsGlobalVariable>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "variables" => {
                            if v.is_null() {
                                continue;
                            }
                            variables = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsListGlobalVariablesResponse {
                    variables,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsListGlobalVariablesResponseVisitor)
    }
}
