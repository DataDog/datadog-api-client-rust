// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about an artifact file or directory within a run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModelLabArtifactObjectInfo {
    /// The size of the file in bytes.
    #[serde(
        rename = "file_size",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub file_size: Option<Option<i64>>,
    /// Whether this artifact entry is a directory.
    #[serde(rename = "is_dir")]
    pub is_dir: bool,
    /// The path of the artifact relative to the run's artifact root.
    #[serde(rename = "path")]
    pub path: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ModelLabArtifactObjectInfo {
    pub fn new(is_dir: bool, path: String) -> ModelLabArtifactObjectInfo {
        ModelLabArtifactObjectInfo {
            file_size: None,
            is_dir,
            path,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn file_size(mut self, value: Option<i64>) -> Self {
        self.file_size = Some(value);
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

impl<'de> Deserialize<'de> for ModelLabArtifactObjectInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ModelLabArtifactObjectInfoVisitor;
        impl<'a> Visitor<'a> for ModelLabArtifactObjectInfoVisitor {
            type Value = ModelLabArtifactObjectInfo;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut file_size: Option<Option<i64>> = None;
                let mut is_dir: Option<bool> = None;
                let mut path: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "file_size" => {
                            file_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_dir" => {
                            is_dir = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path" => {
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let is_dir = is_dir.ok_or_else(|| M::Error::missing_field("is_dir"))?;
                let path = path.ok_or_else(|| M::Error::missing_field("path"))?;

                let content = ModelLabArtifactObjectInfo {
                    file_size,
                    is_dir,
                    path,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ModelLabArtifactObjectInfoVisitor)
    }
}
