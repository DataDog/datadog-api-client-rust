// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboardUpdateRequest {
    /// Timeframe setting for the shared dashboard.
    #[serde(rename = "global_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub global_time: NullableSharedDashboardUpdateRequestGlobalTime,
    /// Whether to allow viewers to select a different global time setting for the shared dashboard.
    #[serde(rename = "global_time_selectable_enabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub global_time_selectable_enabled: Option<Bool>,
    /// List of objects representing template variables on the shared dashboard which can have selectable values.
    #[serde(rename = "selectable_template_vars", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub selectable_template_vars: Vec<SelectableTemplateVariableItems>,
    /// List of email addresses that can be given access to the shared dashboard.
    #[serde(rename = "share_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub share_list: datadog.NullableList[String],
    /// Type of sharing access (either open to anyone who has the public URL or invite-only).
    #[serde(rename = "share_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub share_type: NullableDashboardShareType,
}

