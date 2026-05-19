// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about a project-level artifact file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModelLabArtifactInfo {
    /// The full artifact path relative to the project's artifact root.
    #[serde(rename = "artifact_path")]
    pub artifact_path: String,
    /// The date and time the artifact was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The size of the file in bytes.
    #[serde(
        rename = "file_size",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub file_size: Option<Option<i64>>,
    /// The filename of the artifact.
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ModelLabArtifactInfo {
    pub fn new(
        artifact_path: String,
        created_at: chrono::DateTime<chrono::Utc>,
        filename: String,
    ) -> ModelLabArtifactInfo {
        ModelLabArtifactInfo {
            artifact_path,
            created_at,
            file_size: None,
            filename,
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

impl<'de> Deserialize<'de> for ModelLabArtifactInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ModelLabArtifactInfoVisitor;
        impl<'a> Visitor<'a> for ModelLabArtifactInfoVisitor {
            type Value = ModelLabArtifactInfo;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut artifact_path: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut file_size: Option<Option<i64>> = None;
                let mut filename: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "artifact_path" => {
                            artifact_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_size" => {
                            file_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filename" => {
                            filename = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let artifact_path =
                    artifact_path.ok_or_else(|| M::Error::missing_field("artifact_path"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let filename = filename.ok_or_else(|| M::Error::missing_field("filename"))?;

                let content = ModelLabArtifactInfo {
                    artifact_path,
                    created_at,
                    file_size,
                    filename,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ModelLabArtifactInfoVisitor)
    }
}
