// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An error encountered while validating or applying an operation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsOwnershipMappingBatchError {
    /// A human-readable explanation specific to this error.
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// The HTTP status code applicable to this error.
    #[serde(rename = "status")]
    pub status: String,
    /// A short, human-readable summary of the error.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsOwnershipMappingBatchError {
    pub fn new(status: String, title: String) -> TeamsOwnershipMappingBatchError {
        TeamsOwnershipMappingBatchError {
            detail: None,
            status,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn detail(mut self, value: String) -> Self {
        self.detail = Some(value);
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

impl<'de> Deserialize<'de> for TeamsOwnershipMappingBatchError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsOwnershipMappingBatchErrorVisitor;
        impl<'a> Visitor<'a> for TeamsOwnershipMappingBatchErrorVisitor {
            type Value = TeamsOwnershipMappingBatchError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut detail: Option<String> = None;
                let mut status: Option<String> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "detail" => {
                            if v.is_null() {
                                continue;
                            }
                            detail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = TeamsOwnershipMappingBatchError {
                    detail,
                    status,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamsOwnershipMappingBatchErrorVisitor)
    }
}
