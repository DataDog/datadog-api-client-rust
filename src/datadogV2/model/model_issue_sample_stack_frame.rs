// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A stack frame.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueSampleStackFrame {
    /// Source code around a stack frame. Empty when the source code is unavailable.
    #[serde(rename = "code_snippet")]
    pub code_snippet: Option<crate::datadogV2::model::IssueSampleCodeSnippet>,
    /// Location of a stack frame in a repository file. Empty when the source code is unavailable.
    #[serde(rename = "git_file_location")]
    pub git_file_location: Option<crate::datadogV2::model::IssueSampleGitFileLocation>,
    /// Text of the frame, as it appears in the stack trace.
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueSampleStackFrame {
    pub fn new() -> IssueSampleStackFrame {
        IssueSampleStackFrame {
            code_snippet: None,
            git_file_location: None,
            text: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn code_snippet(mut self, value: crate::datadogV2::model::IssueSampleCodeSnippet) -> Self {
        self.code_snippet = Some(value);
        self
    }

    pub fn git_file_location(
        mut self,
        value: crate::datadogV2::model::IssueSampleGitFileLocation,
    ) -> Self {
        self.git_file_location = Some(value);
        self
    }

    pub fn text(mut self, value: String) -> Self {
        self.text = Some(value);
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

impl Default for IssueSampleStackFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IssueSampleStackFrame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueSampleStackFrameVisitor;
        impl<'a> Visitor<'a> for IssueSampleStackFrameVisitor {
            type Value = IssueSampleStackFrame;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code_snippet: Option<crate::datadogV2::model::IssueSampleCodeSnippet> =
                    None;
                let mut git_file_location: Option<
                    crate::datadogV2::model::IssueSampleGitFileLocation,
                > = None;
                let mut text: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code_snippet" => {
                            if v.is_null() {
                                continue;
                            }
                            code_snippet =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "git_file_location" => {
                            if v.is_null() {
                                continue;
                            }
                            git_file_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text" => {
                            if v.is_null() {
                                continue;
                            }
                            text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IssueSampleStackFrame {
                    code_snippet,
                    git_file_location,
                    text,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueSampleStackFrameVisitor)
    }
}
