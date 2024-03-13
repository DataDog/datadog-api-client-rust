// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A dashboard is Datadog’s tool for visually tracking, analyzing, and displaying
/// key performance metrics, which enable you to monitor the health of your infrastructure.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Dashboard {
    /// Identifier of the dashboard author.
    #[serde(rename = "author_handle")]
    pub author_handle: Option<String>,
    /// Name of the dashboard author.
    #[serde(
        rename = "author_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub author_name: Option<Option<String>>,
    /// Creation date of the dashboard.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Description of the dashboard.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// ID of the dashboard.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether this dashboard is read-only. If True, only the author and admins can make changes to it. Prefer using `restricted_roles` to manage write authorization.
    #[deprecated]
    #[serde(rename = "is_read_only")]
    pub is_read_only: Option<bool>,
    /// Layout type of the dashboard.
    #[serde(rename = "layout_type")]
    pub layout_type: crate::datadogV1::model::DashboardLayoutType,
    /// Modification date of the dashboard.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// List of handles of users to notify when changes are made to this dashboard.
    #[serde(
        rename = "notify_list",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub notify_list: Option<Option<Vec<String>>>,
    /// Reflow type for a **new dashboard layout** dashboard. Set this only when layout type is 'ordered'.
    /// If set to 'fixed', the dashboard expects all widgets to have a layout, and if it's set to 'auto',
    /// widgets should not have layouts.
    #[serde(rename = "reflow_type")]
    pub reflow_type: Option<crate::datadogV1::model::DashboardReflowType>,
    /// A list of role identifiers. Only the author and users associated with at least one of these roles can edit this dashboard.
    #[serde(rename = "restricted_roles")]
    pub restricted_roles: Option<Vec<String>>,
    /// List of team names representing ownership of a dashboard.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// Array of template variables saved views.
    #[serde(
        rename = "template_variable_presets",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub template_variable_presets:
        Option<Option<Vec<crate::datadogV1::model::DashboardTemplateVariablePreset>>>,
    /// List of template variables for this dashboard.
    #[serde(
        rename = "template_variables",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub template_variables: Option<Option<Vec<crate::datadogV1::model::DashboardTemplateVariable>>>,
    /// Title of the dashboard.
    #[serde(rename = "title")]
    pub title: String,
    /// The URL of the dashboard.
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// List of widgets to display on the dashboard.
    #[serde(rename = "widgets")]
    pub widgets: Vec<crate::datadogV1::model::Widget>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Dashboard {
    pub fn new(
        layout_type: crate::datadogV1::model::DashboardLayoutType,
        title: String,
        widgets: Vec<crate::datadogV1::model::Widget>,
    ) -> Dashboard {
        #[allow(deprecated)]
        Dashboard {
            author_handle: None,
            author_name: None,
            created_at: None,
            description: None,
            id: None,
            is_read_only: None,
            layout_type,
            modified_at: None,
            notify_list: None,
            reflow_type: None,
            restricted_roles: None,
            tags: None,
            template_variable_presets: None,
            template_variables: None,
            title,
            url: None,
            widgets,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn author_handle(mut self, value: String) -> Self {
        self.author_handle = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn author_name(mut self, value: Option<String>) -> Self {
        self.author_name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn created_at(mut self, value: String) -> Self {
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
    pub fn modified_at(mut self, value: String) -> Self {
        self.modified_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn notify_list(mut self, value: Option<Vec<String>>) -> Self {
        self.notify_list = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn reflow_type(mut self, value: crate::datadogV1::model::DashboardReflowType) -> Self {
        self.reflow_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn restricted_roles(mut self, value: Vec<String>) -> Self {
        self.restricted_roles = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn tags(mut self, value: Option<Vec<String>>) -> Self {
        self.tags = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn template_variable_presets(
        mut self,
        value: Option<Vec<crate::datadogV1::model::DashboardTemplateVariablePreset>>,
    ) -> Self {
        self.template_variable_presets = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn template_variables(
        mut self,
        value: Option<Vec<crate::datadogV1::model::DashboardTemplateVariable>>,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for Dashboard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardVisitor;
        impl<'a> Visitor<'a> for DashboardVisitor {
            type Value = Dashboard;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author_handle: Option<String> = None;
                let mut author_name: Option<Option<String>> = None;
                let mut created_at: Option<String> = None;
                let mut description: Option<Option<String>> = None;
                let mut id: Option<String> = None;
                let mut is_read_only: Option<bool> = None;
                let mut layout_type: Option<crate::datadogV1::model::DashboardLayoutType> = None;
                let mut modified_at: Option<String> = None;
                let mut notify_list: Option<Option<Vec<String>>> = None;
                let mut reflow_type: Option<crate::datadogV1::model::DashboardReflowType> = None;
                let mut restricted_roles: Option<Vec<String>> = None;
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut template_variable_presets: Option<
                    Option<Vec<crate::datadogV1::model::DashboardTemplateVariablePreset>>,
                > = None;
                let mut template_variables: Option<
                    Option<Vec<crate::datadogV1::model::DashboardTemplateVariable>>,
                > = None;
                let mut title: Option<String> = None;
                let mut url: Option<String> = None;
                let mut widgets: Option<Vec<crate::datadogV1::model::Widget>> = None;
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
                        "author_name" => {
                            author_name =
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
                        "notify_list" => {
                            notify_list =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reflow_type" => {
                            if v.is_null() {
                                continue;
                            }
                            reflow_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reflow_type) = reflow_type {
                                match _reflow_type {
                                    crate::datadogV1::model::DashboardReflowType::UnparsedObject(_reflow_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "restricted_roles" => {
                            if v.is_null() {
                                continue;
                            }
                            restricted_roles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_variable_presets" => {
                            template_variable_presets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_variables" => {
                            template_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "widgets" => {
                            widgets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let layout_type =
                    layout_type.ok_or_else(|| M::Error::missing_field("layout_type"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let widgets = widgets.ok_or_else(|| M::Error::missing_field("widgets"))?;

                let content = Dashboard {
                    author_handle,
                    author_name,
                    created_at,
                    description,
                    id,
                    is_read_only,
                    layout_type,
                    modified_at,
                    notify_list,
                    reflow_type,
                    restricted_roles,
                    tags,
                    template_variable_presets,
                    template_variables,
                    title,
                    url,
                    widgets,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardVisitor)
    }
}
