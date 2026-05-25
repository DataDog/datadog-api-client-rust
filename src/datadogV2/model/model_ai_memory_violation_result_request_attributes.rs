// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an AI memory violation result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiMemoryViolationResultRequestAttributes {
    /// The line number where the violation was found.
    #[serde(rename = "line")]
    pub line: i64,
    /// A message explaining the violation result.
    #[serde(rename = "message")]
    pub message: String,
    /// The file path where the violation was found.
    #[serde(rename = "name")]
    pub name: String,
    /// The repository identifier.
    #[serde(rename = "repository_id")]
    pub repository_id: String,
    /// The rule identifier in the format ruleset/rule.
    #[serde(rename = "rule")]
    pub rule: String,
    /// The git commit SHA where the violation was found.
    #[serde(rename = "sha")]
    pub sha: String,
    /// The type of AI memory violation result indicating whether it is a true positive or false positive.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AiMemoryViolationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiMemoryViolationResultRequestAttributes {
    pub fn new(
        line: i64,
        message: String,
        name: String,
        repository_id: String,
        rule: String,
        sha: String,
        type_: crate::datadogV2::model::AiMemoryViolationType,
    ) -> AiMemoryViolationResultRequestAttributes {
        AiMemoryViolationResultRequestAttributes {
            line,
            message,
            name,
            repository_id,
            rule,
            sha,
            type_,
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

impl<'de> Deserialize<'de> for AiMemoryViolationResultRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiMemoryViolationResultRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AiMemoryViolationResultRequestAttributesVisitor {
            type Value = AiMemoryViolationResultRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut line: Option<i64> = None;
                let mut message: Option<String> = None;
                let mut name: Option<String> = None;
                let mut repository_id: Option<String> = None;
                let mut rule: Option<String> = None;
                let mut sha: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::AiMemoryViolationType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "line" => {
                            line = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_id" => {
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule" => {
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sha" => {
                            sha = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::AiMemoryViolationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let line = line.ok_or_else(|| M::Error::missing_field("line"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let repository_id =
                    repository_id.ok_or_else(|| M::Error::missing_field("repository_id"))?;
                let rule = rule.ok_or_else(|| M::Error::missing_field("rule"))?;
                let sha = sha.ok_or_else(|| M::Error::missing_field("sha"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AiMemoryViolationResultRequestAttributes {
                    line,
                    message,
                    name,
                    repository_id,
                    rule,
                    sha,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiMemoryViolationResultRequestAttributesVisitor)
    }
}
