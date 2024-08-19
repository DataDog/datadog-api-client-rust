// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dashboard definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardSummaryDefinition {
    /// Identifier of the dashboard author.
    #[serde(rename = "author_handle")]
    pub author_handle: Option<String>,
    /// Creation date of the dashboard.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Description of the dashboard.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// Dashboard identifier.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether this dashboard is read-only. If True, only the author and admins can make changes to it.
    ///
    /// This property is deprecated; please use the [Restriction Policies API](<https://docs.datadoghq.com/api/latest/restriction-policies/>) instead to manage write authorization for individual dashboards.
    #[deprecated]
    #[serde(rename = "is_read_only")]
    pub is_read_only: Option<bool>,
    /// Layout type of the dashboard.
    #[serde(rename = "layout_type")]
    pub layout_type: Option<crate::datadogV1::model::DashboardLayoutType>,
    /// Modification date of the dashboard.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Title of the dashboard.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// URL of the dashboard.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardSummaryDefinition {
    pub fn new() -> DashboardSummaryDefinition {
        #[allow(deprecated)]
        DashboardSummaryDefinition {
            author_handle: None,
            created_at: None,
            description: None,
            id: None,
            is_read_only: None,
            layout_type: None,
            modified_at: None,
            title: None,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn author_handle(mut self, value: String) -> Self {
        self.author_handle = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn is_read_only(mut self, value: bool) -> Self {
        self.is_read_only = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn layout_type(mut self, value: crate::datadogV1::model::DashboardLayoutType) -> Self {
        self.layout_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
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

impl Default for DashboardSummaryDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardSummaryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardSummaryDefinitionVisitor;
        impl<'a> Visitor<'a> for DashboardSummaryDefinitionVisitor {
            type Value = DashboardSummaryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author_handle: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<Option<String>> = None;
                let mut id: Option<String> = None;
                let mut is_read_only: Option<bool> = None;
                let mut layout_type: Option<crate::datadogV1::model::DashboardLayoutType> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut title: Option<String> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            author_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_read_only" => {
                            if v.is_null() {
                                continue;
                            }
                            is_read_only =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "layout_type" => {
                            if v.is_null() {
                                continue;
                            }
                            layout_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _layout_type) = layout_type {
                                match _layout_type {
                                    crate::datadogV1::model::DashboardLayoutType::UnparsedObject(_layout_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = DashboardSummaryDefinition {
                    author_handle,
                    created_at,
                    description,
                    id,
                    is_read_only,
                    layout_type,
                    modified_at,
                    title,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardSummaryDefinitionVisitor)
    }
}
