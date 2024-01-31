// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A dashboard is Datadog’s tool for visually tracking, analyzing, and displaying
/// key performance metrics, which enable you to monitor the health of your infrastructure.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }

    #[allow(deprecated)]
    pub fn with_author_handle(&mut self, value: String) -> &mut Self {
        self.author_handle = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_author_name(&mut self, value: Option<String>) -> &mut Self {
        self.author_name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_created_at(&mut self, value: String) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_description(&mut self, value: Option<String>) -> &mut Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_is_read_only(&mut self, value: bool) -> &mut Self {
        self.is_read_only = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_modified_at(&mut self, value: String) -> &mut Self {
        self.modified_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_notify_list(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.notify_list = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_reflow_type(
        &mut self,
        value: crate::datadogV1::model::DashboardReflowType,
    ) -> &mut Self {
        self.reflow_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_restricted_roles(&mut self, value: Vec<String>) -> &mut Self {
        self.restricted_roles = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_tags(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_template_variable_presets(
        &mut self,
        value: Option<Vec<crate::datadogV1::model::DashboardTemplateVariablePreset>>,
    ) -> &mut Self {
        self.template_variable_presets = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_template_variables(
        &mut self,
        value: Option<Vec<crate::datadogV1::model::DashboardTemplateVariable>>,
    ) -> &mut Self {
        self.template_variables = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_url(&mut self, value: String) -> &mut Self {
        self.url = Some(value);
        self
    }
}
