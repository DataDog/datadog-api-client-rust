// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The metadata object associated with how a dashboard has been/will be shared.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboard {
    /// User who shared the dashboard.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV1::model::SharedDashboardAuthor>,
    /// Date the dashboard was shared.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// ID of the dashboard to share.
    #[serde(rename = "dashboard_id")]
    pub dashboard_id: String,
    /// The type of the associated private dashboard.
    #[serde(rename = "dashboard_type")]
    pub dashboard_type: crate::datadogV1::model::DashboardType,
    /// The `SharedDashboard` `embeddable_domains`.
    #[serde(rename = "embeddable_domains")]
    pub embeddable_domains: Option<Vec<String>>,
    /// The time when an OPEN shared dashboard becomes publicly unavailable.
    #[serde(
        rename = "expiration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub expiration: Option<Option<chrono::DateTime<chrono::Utc>>>,
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
    /// The `SharedDashboard` `invitees`.
    #[serde(rename = "invitees")]
    pub invitees: Option<Vec<crate::datadogV1::model::SharedDashboardInviteesItems>>,
    /// The last time the shared dashboard was accessed. Null if never accessed.
    #[serde(
        rename = "last_accessed",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_accessed: Option<Option<chrono::DateTime<chrono::Utc>>>,
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
    #[deprecated]
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
    /// Active means the dashboard is publicly available. Paused means the dashboard is not publicly available.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SharedDashboardStatus>,
    /// Title of the shared dashboard.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// A unique token assigned to the shared dashboard.
    #[serde(rename = "token")]
    pub token: Option<String>,
    /// The viewing preferences for a shared dashboard.
    #[serde(rename = "viewing_preferences")]
    pub viewing_preferences: Option<crate::datadogV1::model::ViewingPreferences>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboard {
    pub fn new(
        dashboard_id: String,
        dashboard_type: crate::datadogV1::model::DashboardType,
    ) -> SharedDashboard {
        #[allow(deprecated)]
        SharedDashboard {
            author: None,
            created: None,
            dashboard_id,
            dashboard_type,
            embeddable_domains: None,
            expiration: None,
            global_time: None,
            global_time_selectable_enabled: None,
            invitees: None,
            last_accessed: None,
            public_url: None,
            selectable_template_vars: None,
            share_list: None,
            share_type: None,
            status: None,
            title: None,
            token: None,
            viewing_preferences: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn author(mut self, value: crate::datadogV1::model::SharedDashboardAuthor) -> Self {
        self.author = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn embeddable_domains(mut self, value: Vec<String>) -> Self {
        self.embeddable_domains = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn expiration(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.expiration = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn global_time(mut self, value: crate::datadogV1::model::DashboardGlobalTime) -> Self {
        self.global_time = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn global_time_selectable_enabled(mut self, value: Option<bool>) -> Self {
        self.global_time_selectable_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn invitees(
        mut self,
        value: Vec<crate::datadogV1::model::SharedDashboardInviteesItems>,
    ) -> Self {
        self.invitees = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn last_accessed(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.last_accessed = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn public_url(mut self, value: String) -> Self {
        self.public_url = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn selectable_template_vars(
        mut self,
        value: Option<Vec<crate::datadogV1::model::SelectableTemplateVariableItems>>,
    ) -> Self {
        self.selectable_template_vars = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn share_list(mut self, value: Option<Vec<String>>) -> Self {
        self.share_list = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn share_type(
        mut self,
        value: Option<crate::datadogV1::model::DashboardShareType>,
    ) -> Self {
        self.share_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status(mut self, value: crate::datadogV1::model::SharedDashboardStatus) -> Self {
        self.status = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn token(mut self, value: String) -> Self {
        self.token = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn viewing_preferences(
        mut self,
        value: crate::datadogV1::model::ViewingPreferences,
    ) -> Self {
        self.viewing_preferences = Some(value);
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

impl<'de> Deserialize<'de> for SharedDashboard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardVisitor;
        impl<'a> Visitor<'a> for SharedDashboardVisitor {
            type Value = SharedDashboard;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV1::model::SharedDashboardAuthor> = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut dashboard_id: Option<String> = None;
                let mut dashboard_type: Option<crate::datadogV1::model::DashboardType> = None;
                let mut embeddable_domains: Option<Vec<String>> = None;
                let mut expiration: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut global_time: Option<crate::datadogV1::model::DashboardGlobalTime> = None;
                let mut global_time_selectable_enabled: Option<Option<bool>> = None;
                let mut invitees: Option<
                    Vec<crate::datadogV1::model::SharedDashboardInviteesItems>,
                > = None;
                let mut last_accessed: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut public_url: Option<String> = None;
                let mut selectable_template_vars: Option<
                    Option<Vec<crate::datadogV1::model::SelectableTemplateVariableItems>>,
                > = None;
                let mut share_list: Option<Option<Vec<String>>> = None;
                let mut share_type: Option<Option<crate::datadogV1::model::DashboardShareType>> =
                    None;
                let mut status: Option<crate::datadogV1::model::SharedDashboardStatus> = None;
                let mut title: Option<String> = None;
                let mut token: Option<String> = None;
                let mut viewing_preferences: Option<crate::datadogV1::model::ViewingPreferences> =
                    None;
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
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dashboard_id" => {
                            dashboard_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dashboard_type" => {
                            dashboard_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _dashboard_type) = dashboard_type {
                                match _dashboard_type {
                                    crate::datadogV1::model::DashboardType::UnparsedObject(
                                        _dashboard_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "embeddable_domains" => {
                            if v.is_null() {
                                continue;
                            }
                            embeddable_domains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expiration" => {
                            expiration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_time" => {
                            if v.is_null() {
                                continue;
                            }
                            global_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_time_selectable_enabled" => {
                            global_time_selectable_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invitees" => {
                            if v.is_null() {
                                continue;
                            }
                            invitees = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_accessed" => {
                            last_accessed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_url" => {
                            if v.is_null() {
                                continue;
                            }
                            public_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selectable_template_vars" => {
                            selectable_template_vars =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "share_list" => {
                            share_list = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "share_type" => {
                            share_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _share_type) = share_type {
                                match _share_type {
                                    Some(
                                        crate::datadogV1::model::DashboardShareType::UnparsedObject(
                                            _share_type,
                                        ),
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::SharedDashboardStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token" => {
                            if v.is_null() {
                                continue;
                            }
                            token = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "viewing_preferences" => {
                            if v.is_null() {
                                continue;
                            }
                            viewing_preferences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let dashboard_id =
                    dashboard_id.ok_or_else(|| M::Error::missing_field("dashboard_id"))?;
                let dashboard_type =
                    dashboard_type.ok_or_else(|| M::Error::missing_field("dashboard_type"))?;

                #[allow(deprecated)]
                let content = SharedDashboard {
                    author,
                    created,
                    dashboard_id,
                    dashboard_type,
                    embeddable_domains,
                    expiration,
                    global_time,
                    global_time_selectable_enabled,
                    invitees,
                    last_accessed,
                    public_url,
                    selectable_template_vars,
                    share_list,
                    share_type,
                    status,
                    title,
                    token,
                    viewing_preferences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardVisitor)
    }
}
