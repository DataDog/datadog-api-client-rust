// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Represents a single validation error, including a human-readable title and metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ValidationError {
    /// Describes additional metadata for validation errors, including field names and error messages.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::ValidationErrorMeta,
    /// A short, human-readable summary of the error.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ValidationError {
    pub fn new(
        meta: crate::datadogV2::model::ValidationErrorMeta,
        title: String,
    ) -> ValidationError {
        ValidationError {
            meta,
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

impl<'de> Deserialize<'de> for ValidationError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValidationErrorVisitor;
        impl<'a> Visitor<'a> for ValidationErrorVisitor {
            type Value = ValidationError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut meta: Option<crate::datadogV2::model::ValidationErrorMeta> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "meta" => {
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let meta = meta.ok_or_else(|| M::Error::missing_field("meta"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = ValidationError {
                    meta,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ValidationErrorVisitor)
    }
}
