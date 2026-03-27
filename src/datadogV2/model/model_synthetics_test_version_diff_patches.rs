// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a patch in the diff.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestVersionDiffPatches {
    /// List of individual diff operations.
    #[serde(rename = "diffs")]
    pub diffs: Option<Vec<crate::datadogV2::model::SyntheticsTestVersionDiffPatchDiff>>,
    /// Length of the original text segment.
    #[serde(rename = "length1")]
    pub length1: Option<i64>,
    /// Length of the modified text segment.
    #[serde(rename = "length2")]
    pub length2: Option<i64>,
    /// Start position in the original text.
    #[serde(rename = "start1")]
    pub start1: Option<i64>,
    /// Start position in the modified text.
    #[serde(rename = "start2")]
    pub start2: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestVersionDiffPatches {
    pub fn new() -> SyntheticsTestVersionDiffPatches {
        SyntheticsTestVersionDiffPatches {
            diffs: None,
            length1: None,
            length2: None,
            start1: None,
            start2: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn diffs(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestVersionDiffPatchDiff>,
    ) -> Self {
        self.diffs = Some(value);
        self
    }

    pub fn length1(mut self, value: i64) -> Self {
        self.length1 = Some(value);
        self
    }

    pub fn length2(mut self, value: i64) -> Self {
        self.length2 = Some(value);
        self
    }

    pub fn start1(mut self, value: i64) -> Self {
        self.start1 = Some(value);
        self
    }

    pub fn start2(mut self, value: i64) -> Self {
        self.start2 = Some(value);
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

impl Default for SyntheticsTestVersionDiffPatches {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestVersionDiffPatches {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestVersionDiffPatchesVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestVersionDiffPatchesVisitor {
            type Value = SyntheticsTestVersionDiffPatches;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut diffs: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestVersionDiffPatchDiff>,
                > = None;
                let mut length1: Option<i64> = None;
                let mut length2: Option<i64> = None;
                let mut start1: Option<i64> = None;
                let mut start2: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "diffs" => {
                            if v.is_null() {
                                continue;
                            }
                            diffs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "length1" => {
                            if v.is_null() {
                                continue;
                            }
                            length1 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "length2" => {
                            if v.is_null() {
                                continue;
                            }
                            length2 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start1" => {
                            if v.is_null() {
                                continue;
                            }
                            start1 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start2" => {
                            if v.is_null() {
                                continue;
                            }
                            start2 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestVersionDiffPatches {
                    diffs,
                    length1,
                    length2,
                    start1,
                    start2,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestVersionDiffPatchesVisitor)
    }
}
