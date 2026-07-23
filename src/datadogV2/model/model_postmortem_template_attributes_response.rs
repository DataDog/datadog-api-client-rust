// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a postmortem template returned in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PostmortemTemplateAttributesResponse {
    /// Settings for a postmortem template stored in Confluence. Required when `location` is `confluence`.
    #[serde(rename = "confluence_postmortem_settings")]
    pub confluence_postmortem_settings:
        Option<crate::datadogV2::model::ConfluencePostmortemSettings>,
    /// The templated content of the postmortem, supporting Markdown and incident template variables.
    #[serde(rename = "content")]
    pub content: String,
    /// When the template was created.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Settings for a postmortem template stored in Google Docs. Required when `location` is `google_docs`.
    #[serde(rename = "google_docs_postmortem_settings")]
    pub google_docs_postmortem_settings:
        Option<crate::datadogV2::model::GoogleDocsPostmortemSettings>,
    /// When set, marks this template as a default. The effective default for an incident type is the template with the most recent `is_default` timestamp.
    #[serialize_always]
    #[serde(rename = "is_default")]
    pub is_default: Option<chrono::DateTime<chrono::Utc>>,
    /// The location where the postmortem is created and stored.
    #[serde(rename = "location")]
    pub location: crate::datadogV2::model::PostmortemTemplateLocation,
    /// When the template was last modified.
    #[serde(rename = "modifiedAt")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The name of the template.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PostmortemTemplateAttributesResponse {
    pub fn new(
        content: String,
        created_at: chrono::DateTime<chrono::Utc>,
        is_default: Option<chrono::DateTime<chrono::Utc>>,
        location: crate::datadogV2::model::PostmortemTemplateLocation,
        modified_at: chrono::DateTime<chrono::Utc>,
        name: String,
    ) -> PostmortemTemplateAttributesResponse {
        PostmortemTemplateAttributesResponse {
            confluence_postmortem_settings: None,
            content,
            created_at,
            google_docs_postmortem_settings: None,
            is_default,
            location,
            modified_at,
            name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn confluence_postmortem_settings(
        mut self,
        value: crate::datadogV2::model::ConfluencePostmortemSettings,
    ) -> Self {
        self.confluence_postmortem_settings = Some(value);
        self
    }

    pub fn google_docs_postmortem_settings(
        mut self,
        value: crate::datadogV2::model::GoogleDocsPostmortemSettings,
    ) -> Self {
        self.google_docs_postmortem_settings = Some(value);
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

impl<'de> Deserialize<'de> for PostmortemTemplateAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PostmortemTemplateAttributesResponseVisitor;
        impl<'a> Visitor<'a> for PostmortemTemplateAttributesResponseVisitor {
            type Value = PostmortemTemplateAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confluence_postmortem_settings: Option<
                    crate::datadogV2::model::ConfluencePostmortemSettings,
                > = None;
                let mut content: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut google_docs_postmortem_settings: Option<
                    crate::datadogV2::model::GoogleDocsPostmortemSettings,
                > = None;
                let mut is_default: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut location: Option<crate::datadogV2::model::PostmortemTemplateLocation> =
                    None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confluence_postmortem_settings" => {
                            if v.is_null() {
                                continue;
                            }
                            confluence_postmortem_settings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdAt" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "google_docs_postmortem_settings" => {
                            if v.is_null() {
                                continue;
                            }
                            google_docs_postmortem_settings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_default" => {
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "location" => {
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _location) = location {
                                match _location {
                                    crate::datadogV2::model::PostmortemTemplateLocation::UnparsedObject(_location) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "modifiedAt" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let is_default = is_default.ok_or_else(|| M::Error::missing_field("is_default"))?;
                let location = location.ok_or_else(|| M::Error::missing_field("location"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = PostmortemTemplateAttributesResponse {
                    confluence_postmortem_settings,
                    content,
                    created_at,
                    google_docs_postmortem_settings,
                    is_default,
                    location,
                    modified_at,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PostmortemTemplateAttributesResponseVisitor)
    }
}
