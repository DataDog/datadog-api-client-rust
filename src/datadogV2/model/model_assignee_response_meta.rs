// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Per-finding warnings and failures produced while processing the bulk assignee request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AssigneeResponseMeta {
    /// Findings that could not be assigned or unassigned.
    #[serde(rename = "failures")]
    pub failures: Option<Vec<crate::datadogV2::model::AssignmentResult>>,
    /// Findings for which the assignment succeeded but a non-critical error occurred during processing.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV2::model::AssignmentResult>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AssigneeResponseMeta {
    pub fn new() -> AssigneeResponseMeta {
        AssigneeResponseMeta {
            failures: None,
            warnings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn failures(mut self, value: Vec<crate::datadogV2::model::AssignmentResult>) -> Self {
        self.failures = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<crate::datadogV2::model::AssignmentResult>) -> Self {
        self.warnings = Some(value);
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

impl Default for AssigneeResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AssigneeResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AssigneeResponseMetaVisitor;
        impl<'a> Visitor<'a> for AssigneeResponseMetaVisitor {
            type Value = AssigneeResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut failures: Option<Vec<crate::datadogV2::model::AssignmentResult>> = None;
                let mut warnings: Option<Vec<crate::datadogV2::model::AssignmentResult>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "failures" => {
                            if v.is_null() {
                                continue;
                            }
                            failures = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warnings" => {
                            if v.is_null() {
                                continue;
                            }
                            warnings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AssigneeResponseMeta {
                    failures,
                    warnings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AssigneeResponseMetaVisitor)
    }
}
