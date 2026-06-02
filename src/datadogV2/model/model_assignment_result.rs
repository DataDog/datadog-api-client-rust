// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Per-finding outcome of an assign or unassign operation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AssignmentResult {
    /// Human-readable explanation of the outcome.
    #[serde(rename = "detail")]
    pub detail: String,
    /// Unique identifier of the security finding.
    #[serde(rename = "finding_id")]
    pub finding_id: String,
    /// HTTP-like status code describing the outcome for this finding.
    #[serde(rename = "status")]
    pub status: i32,
    /// Short label describing the outcome for this finding.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AssignmentResult {
    pub fn new(detail: String, finding_id: String, status: i32, title: String) -> AssignmentResult {
        AssignmentResult {
            detail,
            finding_id,
            status,
            title,
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

impl<'de> Deserialize<'de> for AssignmentResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AssignmentResultVisitor;
        impl<'a> Visitor<'a> for AssignmentResultVisitor {
            type Value = AssignmentResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut detail: Option<String> = None;
                let mut finding_id: Option<String> = None;
                let mut status: Option<i32> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "detail" => {
                            detail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "finding_id" => {
                            finding_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let detail = detail.ok_or_else(|| M::Error::missing_field("detail"))?;
                let finding_id = finding_id.ok_or_else(|| M::Error::missing_field("finding_id"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = AssignmentResult {
                    detail,
                    finding_id,
                    status,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AssignmentResultVisitor)
    }
}
