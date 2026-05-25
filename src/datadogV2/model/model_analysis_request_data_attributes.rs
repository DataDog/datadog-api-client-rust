// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the analysis request, containing the source code and rules to apply.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisRequestDataAttributes {
    /// The base64-encoded source code to analyze.
    #[serde(rename = "code")]
    pub code: String,
    /// The encoding of the source code file (must be `utf-8`).
    #[serde(rename = "file_encoding")]
    pub file_encoding: String,
    /// The name of the file being analyzed.
    #[serde(rename = "filename")]
    pub filename: String,
    /// The programming language of the source code.
    #[serde(rename = "language")]
    pub language: String,
    /// The list of static analysis rules to apply during analysis.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::datadogV2::model::AnalysisRequestRule>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnalysisRequestDataAttributes {
    pub fn new(
        code: String,
        file_encoding: String,
        filename: String,
        language: String,
        rules: Vec<crate::datadogV2::model::AnalysisRequestRule>,
    ) -> AnalysisRequestDataAttributes {
        AnalysisRequestDataAttributes {
            code,
            file_encoding,
            filename,
            language,
            rules,
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

impl<'de> Deserialize<'de> for AnalysisRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnalysisRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for AnalysisRequestDataAttributesVisitor {
            type Value = AnalysisRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<String> = None;
                let mut file_encoding: Option<String> = None;
                let mut filename: Option<String> = None;
                let mut language: Option<String> = None;
                let mut rules: Option<Vec<crate::datadogV2::model::AnalysisRequestRule>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_encoding" => {
                            file_encoding =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filename" => {
                            filename = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let code = code.ok_or_else(|| M::Error::missing_field("code"))?;
                let file_encoding =
                    file_encoding.ok_or_else(|| M::Error::missing_field("file_encoding"))?;
                let filename = filename.ok_or_else(|| M::Error::missing_field("filename"))?;
                let language = language.ok_or_else(|| M::Error::missing_field("language"))?;
                let rules = rules.ok_or_else(|| M::Error::missing_field("rules"))?;

                let content = AnalysisRequestDataAttributes {
                    code,
                    file_encoding,
                    filename,
                    language,
                    rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnalysisRequestDataAttributesVisitor)
    }
}
