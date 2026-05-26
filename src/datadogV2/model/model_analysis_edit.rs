// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single edit operation within a fix suggestion for a rule violation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisEdit {
    /// The content to insert or replace at the specified position, if applicable.
    #[serialize_always]
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// The type of code edit to apply when fixing a violation.
    #[serde(rename = "edit_type")]
    pub edit_type: crate::datadogV2::model::AnalysisEditType,
    /// A position in source code, identified by line and column numbers.
    #[serde(rename = "end")]
    pub end: crate::datadogV2::model::AnalysisPosition,
    /// A position in source code, identified by line and column numbers.
    #[serde(rename = "start")]
    pub start: crate::datadogV2::model::AnalysisPosition,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnalysisEdit {
    pub fn new(
        content: Option<String>,
        edit_type: crate::datadogV2::model::AnalysisEditType,
        end: crate::datadogV2::model::AnalysisPosition,
        start: crate::datadogV2::model::AnalysisPosition,
    ) -> AnalysisEdit {
        AnalysisEdit {
            content,
            edit_type,
            end,
            start,
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

impl<'de> Deserialize<'de> for AnalysisEdit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnalysisEditVisitor;
        impl<'a> Visitor<'a> for AnalysisEditVisitor {
            type Value = AnalysisEdit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<Option<String>> = None;
                let mut edit_type: Option<crate::datadogV2::model::AnalysisEditType> = None;
                let mut end: Option<crate::datadogV2::model::AnalysisPosition> = None;
                let mut start: Option<crate::datadogV2::model::AnalysisPosition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "edit_type" => {
                            edit_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _edit_type) = edit_type {
                                match _edit_type {
                                    crate::datadogV2::model::AnalysisEditType::UnparsedObject(
                                        _edit_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let edit_type = edit_type.ok_or_else(|| M::Error::missing_field("edit_type"))?;
                let end = end.ok_or_else(|| M::Error::missing_field("end"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;

                let content = AnalysisEdit {
                    content,
                    edit_type,
                    end,
                    start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnalysisEditVisitor)
    }
}
