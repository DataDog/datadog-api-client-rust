// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Artifact listing for a Model Lab project.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModelLabProjectArtifactsAttributes {
    /// The list of artifact files associated with the project.
    #[serde(rename = "files")]
    pub files: Vec<crate::datadogV2::model::ModelLabArtifactInfo>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ModelLabProjectArtifactsAttributes {
    pub fn new(
        files: Vec<crate::datadogV2::model::ModelLabArtifactInfo>,
    ) -> ModelLabProjectArtifactsAttributes {
        ModelLabProjectArtifactsAttributes {
            files,
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

impl<'de> Deserialize<'de> for ModelLabProjectArtifactsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ModelLabProjectArtifactsAttributesVisitor;
        impl<'a> Visitor<'a> for ModelLabProjectArtifactsAttributesVisitor {
            type Value = ModelLabProjectArtifactsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut files: Option<Vec<crate::datadogV2::model::ModelLabArtifactInfo>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "files" => {
                            files = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let files = files.ok_or_else(|| M::Error::missing_field("files"))?;

                let content = ModelLabProjectArtifactsAttributes {
                    files,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ModelLabProjectArtifactsAttributesVisitor)
    }
}
