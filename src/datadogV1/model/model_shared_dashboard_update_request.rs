// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Update a shared dashboard's settings.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboardUpdateRequest {
    /// Timeframe setting for the shared dashboard.
    #[serde(rename = "global_time")]
    pub global_time: Option<Box<crate::datadogV1::model::SharedDashboardUpdateRequestGlobalTime>>,
    /// Whether to allow viewers to select a different global time setting for the shared dashboard.
    #[serde(
        rename = "global_time_selectable_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub global_time_selectable_enabled: Option<Option<bool>>,
    /// List of objects representing template variables on the shared dashboard which can have selectable values.
    #[serde(
        rename = "selectable_template_vars",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub selectable_template_vars:
        Option<Option<Vec<Option<crate::datadogV1::model::SelectableTemplateVariableItems>>>>,
    /// List of email addresses that can be given access to the shared dashboard.
    #[serde(
        rename = "share_list",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub share_list: Option<Option<Vec<String>>>,
    /// Type of sharing access (either open to anyone who has the public URL or invite-only).
    #[serde(
        rename = "share_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub share_type: Option<Option<crate::datadogV1::model::DashboardShareType>>,
}

impl SharedDashboardUpdateRequest {
    pub fn new(
        global_time: Option<Box<crate::datadogV1::model::SharedDashboardUpdateRequestGlobalTime>>,
    ) -> SharedDashboardUpdateRequest {
        SharedDashboardUpdateRequest {
            global_time,
            global_time_selectable_enabled: None,
            selectable_template_vars: None,
            share_list: None,
            share_type: None,
        }
    }
}