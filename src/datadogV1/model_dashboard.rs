// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dashboard {
    /// Identifier of the dashboard author.
    #[serde(rename = "author_handle", skip_serializing_if = "Option::is_none")]
    pub author_handle: String,
    /// Name of the dashboard author.
    #[serde(rename = "author_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub author_name: Option<String>,
    /// Creation date of the dashboard.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// Description of the dashboard.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// ID of the dashboard.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Whether this dashboard is read-only. If True, only the author and admins can make changes to it. Prefer using `restricted_roles` to manage write authorization.
    #[serde(rename = "is_read_only", skip_serializing_if = "Option::is_none")]
    pub is_read_only: bool,
    /// Layout type of the dashboard.
    #[serde(rename = "layout_type", skip_serializing_if = "Option::is_none")]
    pub layout_type: DashboardLayoutType,
    /// Modification date of the dashboard.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: String,
    /// List of handles of users to notify when changes are made to this dashboard.
    #[serde(rename = "notify_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub notify_list: datadog.NullableList[String],
    /// Reflow type for a **new dashboard layout** dashboard. Set this only when layout type is 'ordered'.
If set to 'fixed', the dashboard expects all widgets to have a layout, and if it's set to 'auto',
widgets should not have layouts.
    #[serde(rename = "reflow_type", skip_serializing_if = "Option::is_none")]
    pub reflow_type: DashboardReflowType,
    /// A list of role identifiers. Only the author and users associated with at least one of these roles can edit this dashboard.
    #[serde(rename = "restricted_roles", skip_serializing_if = "Option::is_none")]
    pub restricted_roles: Vec<String>,
    /// List of team names representing ownership of a dashboard.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub tags: datadog.NullableList[String],
    /// Array of template variables saved views.
    #[serde(rename = "template_variable_presets", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub template_variable_presets: Vec<DashboardTemplateVariablePreset>,
    /// List of template variables for this dashboard.
    #[serde(rename = "template_variables", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub template_variables: Vec<DashboardTemplateVariable>,
    /// Title of the dashboard.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// The URL of the dashboard.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
    /// List of widgets to display on the dashboard.
    #[serde(rename = "widgets", skip_serializing_if = "Option::is_none")]
    pub widgets: Vec<Widget>,
}

