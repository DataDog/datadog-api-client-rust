// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A fix suggestion for a rule violation, consisting of one or more edit operations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisFix {
    /// A human-readable description of what the fix does.
    #[serde(rename = "description")]
    pub description: String,
    /// The list of edit operations that constitute the fix.
    #[serde(rename = "edits")]
    pub edits: Vec<crate::datadogV2::model::AnalysisEdit>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnalysisFix {
    pub fn new(
        description: String,
        edits: Vec<crate::datadogV2::model::AnalysisEdit>,
    ) -> AnalysisFix {
        AnalysisFix {
            description,
            edits,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AnalysisFix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnalysisFixVisitor;
        impl<'a> Visitor<'a> for AnalysisFixVisitor {
            type Value = AnalysisFix;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut edits: Option<Vec<crate::datadogV2::model::AnalysisEdit>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "edits" => {
                            edits = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let edits = edits.ok_or_else(|| M::Error::missing_field("edits"))?;

                let content = AnalysisFix {
                    description,
                    edits,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnalysisFixVisitor)
    }
}
