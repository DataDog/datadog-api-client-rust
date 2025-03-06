// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `WorkflowDataUpdateAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WorkflowDataUpdateAttributes {
    /// When the workflow was created.
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Description of the workflow.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Name of the workflow.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Set the workflow to published or unpublished. Workflows in an unpublished state will only be executable via manual runs. Automatic triggers such as Schedule will not execute the workflow until it is published.
    #[serde(rename = "published")]
    pub published: Option<bool>,
    /// The spec defines what the workflow does.
    #[serde(rename = "spec")]
    pub spec: Option<crate::datadogV2::model::Spec>,
    /// Tags of the workflow.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// When the workflow was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// If a Webhook trigger is defined on this workflow, a webhookSecret is required and should be provided here.
    #[serde(rename = "webhookSecret")]
    pub webhook_secret: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WorkflowDataUpdateAttributes {
    pub fn new() -> WorkflowDataUpdateAttributes {
        WorkflowDataUpdateAttributes {
            created_at: None,
            description: None,
            name: None,
            published: None,
            spec: None,
            tags: None,
            updated_at: None,
            webhook_secret: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn published(mut self, value: bool) -> Self {
        self.published = Some(value);
        self
    }

    pub fn spec(mut self, value: crate::datadogV2::model::Spec) -> Self {
        self.spec = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn webhook_secret(mut self, value: String) -> Self {
        self.webhook_secret = Some(value);
        self
    }
}

impl Default for WorkflowDataUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WorkflowDataUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WorkflowDataUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for WorkflowDataUpdateAttributesVisitor {
            type Value = WorkflowDataUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut published: Option<bool> = None;
                let mut spec: Option<crate::datadogV2::model::Spec> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut webhook_secret: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "createdAt" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "published" => {
                            if v.is_null() {
                                continue;
                            }
                            published = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spec" => {
                            if v.is_null() {
                                continue;
                            }
                            spec = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedAt" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "webhookSecret" => {
                            if v.is_null() {
                                continue;
                            }
                            webhook_secret =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = WorkflowDataUpdateAttributes {
                    created_at,
                    description,
                    name,
                    published,
                    spec,
                    tags,
                    updated_at,
                    webhook_secret,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WorkflowDataUpdateAttributesVisitor)
    }
}
