// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a specific version of an LLM Observability prompt.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPromptVersionDataAttributes {
    /// UUID of the user who authored this version.
    #[serde(rename = "author")]
    pub author: Option<String>,
    /// Timestamp stored on this prompt version.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Datasets observed in runs associated with this prompt version.
    #[serde(rename = "datasets")]
    pub datasets: Option<Vec<crate::datadogV2::model::LLMObsPromptDataset>>,
    /// Description of this version.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Labels attached to this version (for example `development`, `staging`, `production`).
    #[deprecated]
    #[serde(rename = "labels")]
    pub labels: Option<Vec<String>>,
    /// Timestamp of the most recent observed run of this prompt version.
    #[serde(rename = "last_seen_at")]
    pub last_seen_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The ML application this prompt is associated with.
    #[serde(rename = "ml_app")]
    pub ml_app: Option<String>,
    /// ML applications observed running this prompt version.
    #[serde(rename = "ml_apps")]
    pub ml_apps: Option<Vec<String>>,
    /// Customer-provided identifier of the parent prompt.
    #[serde(rename = "prompt_id")]
    pub prompt_id: String,
    /// Unique identifier of the parent prompt.
    #[serde(rename = "prompt_uuid")]
    pub prompt_uuid: String,
    /// Tags observed on runs of this prompt version.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// A text template or a list of chat messages.
    #[serde(rename = "template")]
    pub template: crate::datadogV2::model::LLMObsPromptTemplate,
    /// User-supplied identifier for this version.
    #[serde(rename = "user_version")]
    pub user_version: Option<String>,
    /// Sequential version number.
    #[serde(rename = "version")]
    pub version: i64,
    /// Timestamp when this version was created.
    #[serde(rename = "version_created_at")]
    pub version_created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPromptVersionDataAttributes {
    pub fn new(
        prompt_id: String,
        prompt_uuid: String,
        template: crate::datadogV2::model::LLMObsPromptTemplate,
        version: i64,
    ) -> LLMObsPromptVersionDataAttributes {
        #[allow(deprecated)]
        LLMObsPromptVersionDataAttributes {
            author: None,
            created_at: None,
            datasets: None,
            description: None,
            labels: None,
            last_seen_at: None,
            ml_app: None,
            ml_apps: None,
            prompt_id,
            prompt_uuid,
            tags: None,
            template,
            user_version: None,
            version,
            version_created_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn author(mut self, value: String) -> Self {
        self.author = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn datasets(mut self, value: Vec<crate::datadogV2::model::LLMObsPromptDataset>) -> Self {
        self.datasets = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn last_seen_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ml_app(mut self, value: String) -> Self {
        self.ml_app = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ml_apps(mut self, value: Vec<String>) -> Self {
        self.ml_apps = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn user_version(mut self, value: String) -> Self {
        self.user_version = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn version_created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.version_created_at = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsPromptVersionDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPromptVersionDataAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsPromptVersionDataAttributesVisitor {
            type Value = LLMObsPromptVersionDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut datasets: Option<Vec<crate::datadogV2::model::LLMObsPromptDataset>> = None;
                let mut description: Option<String> = None;
                let mut labels: Option<Vec<String>> = None;
                let mut last_seen_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut ml_app: Option<String> = None;
                let mut ml_apps: Option<Vec<String>> = None;
                let mut prompt_id: Option<String> = None;
                let mut prompt_uuid: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut template: Option<crate::datadogV2::model::LLMObsPromptTemplate> = None;
                let mut user_version: Option<String> = None;
                let mut version: Option<i64> = None;
                let mut version_created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datasets" => {
                            if v.is_null() {
                                continue;
                            }
                            datasets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "labels" => {
                            if v.is_null() {
                                continue;
                            }
                            labels = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_seen_at" => {
                            if v.is_null() {
                                continue;
                            }
                            last_seen_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ml_app" => {
                            if v.is_null() {
                                continue;
                            }
                            ml_app = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ml_apps" => {
                            if v.is_null() {
                                continue;
                            }
                            ml_apps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prompt_id" => {
                            prompt_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prompt_uuid" => {
                            prompt_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template" => {
                            template = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _template) = template {
                                match _template {
                                    crate::datadogV2::model::LLMObsPromptTemplate::UnparsedObject(_template) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "user_version" => {
                            if v.is_null() {
                                continue;
                            }
                            user_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            version_created_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let prompt_id = prompt_id.ok_or_else(|| M::Error::missing_field("prompt_id"))?;
                let prompt_uuid =
                    prompt_uuid.ok_or_else(|| M::Error::missing_field("prompt_uuid"))?;
                let template = template.ok_or_else(|| M::Error::missing_field("template"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                #[allow(deprecated)]
                let content = LLMObsPromptVersionDataAttributes {
                    author,
                    created_at,
                    datasets,
                    description,
                    labels,
                    last_seen_at,
                    ml_app,
                    ml_apps,
                    prompt_id,
                    prompt_uuid,
                    tags,
                    template,
                    user_version,
                    version,
                    version_created_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPromptVersionDataAttributesVisitor)
    }
}
