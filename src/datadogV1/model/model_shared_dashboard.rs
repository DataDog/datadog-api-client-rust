// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata object associated with how a dashboard has been/will be shared.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboard {
    /// User who shared the dashboard.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV1::model::SharedDashboardAuthor>,
    /// Date the dashboard was shared.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// ID of the dashboard to share.
    #[serde(rename = "dashboard_id")]
    pub dashboard_id: String,
    /// The type of the associated private dashboard.
    #[serde(rename = "dashboard_type")]
    pub dashboard_type: crate::datadogV1::model::DashboardType,
    /// Object containing the live span selection for the dashboard.
    #[serde(rename = "global_time")]
    pub global_time: Option<crate::datadogV1::model::DashboardGlobalTime>,
    /// Whether to allow viewers to select a different global time setting for the shared dashboard.
    #[serde(
        rename = "global_time_selectable_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub global_time_selectable_enabled: Option<Option<bool>>,
    /// URL of the shared dashboard.
    #[serde(rename = "public_url")]
    pub public_url: Option<String>,
    /// List of objects representing template variables on the shared dashboard which can have selectable values.
    #[serde(
        rename = "selectable_template_vars",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub selectable_template_vars:
        Option<Option<Vec<crate::datadogV1::model::SelectableTemplateVariableItems>>>,
    /// List of email addresses that can receive an invitation to access to the shared dashboard.
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
    /// A unique token assigned to the shared dashboard.
    #[serde(rename = "token")]
    pub token: Option<String>,
}

impl SharedDashboard {
    pub fn new(
        dashboard_id: String,
        dashboard_type: crate::datadogV1::model::DashboardType,
    ) -> SharedDashboard {
        SharedDashboard {
            author: None,
            created_at: None,
            dashboard_id,
            dashboard_type,
            global_time: None,
            global_time_selectable_enabled: None,
            public_url: None,
            selectable_template_vars: None,
            share_list: None,
            share_type: None,
            token: None,
        }
    }

    pub fn author(&mut self, value: crate::datadogV1::model::SharedDashboardAuthor) -> &mut Self {
        self.author = Some(value);
        self
    }

    pub fn created_at(&mut self, value: String) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    pub fn global_time(
        &mut self,
        value: crate::datadogV1::model::DashboardGlobalTime,
    ) -> &mut Self {
        self.global_time = Some(value);
        self
    }

    pub fn global_time_selectable_enabled(&mut self, value: Option<bool>) -> &mut Self {
        self.global_time_selectable_enabled = Some(value);
        self
    }

    pub fn public_url(&mut self, value: String) -> &mut Self {
        self.public_url = Some(value);
        self
    }

    pub fn selectable_template_vars(
        &mut self,
        value: Option<Vec<crate::datadogV1::model::SelectableTemplateVariableItems>>,
    ) -> &mut Self {
        self.selectable_template_vars = Some(value);
        self
    }

    pub fn share_list(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.share_list = Some(value);
        self
    }

    pub fn share_type(
        &mut self,
        value: Option<crate::datadogV1::model::DashboardShareType>,
    ) -> &mut Self {
        self.share_type = Some(value);
        self
    }

    pub fn token(&mut self, value: String) -> &mut Self {
        self.token = Some(value);
        self
    }
}
