// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a single text diff operation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestVersionDiffPatchDiff {
    /// The text that was changed.
    #[serde(rename = "change_text")]
    pub change_text: Option<String>,
    /// The diff operation applied.
    #[serde(rename = "operation")]
    pub operation: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestVersionDiffPatchDiff {
    pub fn new() -> SyntheticsTestVersionDiffPatchDiff {
        SyntheticsTestVersionDiffPatchDiff {
            change_text: None,
            operation: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn change_text(mut self, value: String) -> Self {
        self.change_text = Some(value);
        self
    }

    pub fn operation(mut self, value: String) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SyntheticsTestVersionDiffPatchDiff {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestVersionDiffPatchDiff {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestVersionDiffPatchDiffVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestVersionDiffPatchDiffVisitor {
            type Value = SyntheticsTestVersionDiffPatchDiff;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_text: Option<String> = None;
                let mut operation: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_text" => {
                            if v.is_null() {
                                continue;
                            }
                            change_text =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation" => {
                            if v.is_null() {
                                continue;
                            }
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestVersionDiffPatchDiff {
                    change_text,
                    operation,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestVersionDiffPatchDiffVisitor)
    }
}
