// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Source code location of a stack frame.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueSampleFrameLink {
    /// Location of a stack frame in a repository file. Empty when the source code is unavailable.
    #[serde(rename = "code_location")]
    pub code_location: crate::datadogV2::model::IssueSampleGitFileLocation,
    /// Index of the frame in the stack trace, starting at 0.
    #[serde(rename = "frame_id")]
    pub frame_id: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueSampleFrameLink {
    pub fn new(
        code_location: crate::datadogV2::model::IssueSampleGitFileLocation,
        frame_id: i64,
    ) -> IssueSampleFrameLink {
        IssueSampleFrameLink {
            code_location,
            frame_id,
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

impl<'de> Deserialize<'de> for IssueSampleFrameLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueSampleFrameLinkVisitor;
        impl<'a> Visitor<'a> for IssueSampleFrameLinkVisitor {
            type Value = IssueSampleFrameLink;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code_location: Option<crate::datadogV2::model::IssueSampleGitFileLocation> =
                    None;
                let mut frame_id: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code_location" => {
                            code_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "frame_id" => {
                            frame_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let code_location =
                    code_location.ok_or_else(|| M::Error::missing_field("code_location"))?;
                let frame_id = frame_id.ok_or_else(|| M::Error::missing_field("frame_id"))?;

                let content = IssueSampleFrameLink {
                    code_location,
                    frame_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueSampleFrameLinkVisitor)
    }
}
