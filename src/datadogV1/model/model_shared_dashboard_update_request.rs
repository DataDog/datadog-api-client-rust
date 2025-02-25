// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Update a shared dashboard's settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboardUpdateRequest {
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
    /// Timeframe setting for the shared dashboard.
    #[serde(
        rename = "global_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub global_time:
        Option<Option<crate::datadogV1::model::SharedDashboardUpdateRequestGlobalTime>>,
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
    /// List of objects representing template variables on the shared dashboard which can have selectable values.
    #[serde(
        rename = "selectable_template_vars",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub selectable_template_vars:
        Option<Option<Vec<crate::datadogV1::model::SelectableTemplateVariableItems>>>,
    /// List of email addresses that can be given access to the shared dashboard.
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
    /// The viewing preferences for a shared dashboard.
    #[serde(rename = "viewing_preferences")]
    pub viewing_preferences: Option<crate::datadogV1::model::ViewingPreferences>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardUpdateRequest {
    pub fn new() -> SharedDashboardUpdateRequest {
        #[allow(deprecated)]
        SharedDashboardUpdateRequest {
            embeddable_domains: None,
            expiration: None,
            global_time: None,
            global_time_selectable_enabled: None,
            invitees: None,
            selectable_template_vars: None,
            share_list: None,
            share_type: None,
            status: None,
            title: None,
            viewing_preferences: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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
    pub fn global_time(
        mut self,
        value: Option<crate::datadogV1::model::SharedDashboardUpdateRequestGlobalTime>,
    ) -> Self {
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

impl Default for SharedDashboardUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SharedDashboardUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardUpdateRequestVisitor;
        impl<'a> Visitor<'a> for SharedDashboardUpdateRequestVisitor {
            type Value = SharedDashboardUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut embeddable_domains: Option<Vec<String>> = None;
                let mut expiration: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut global_time: Option<
                    Option<crate::datadogV1::model::SharedDashboardUpdateRequestGlobalTime>,
                > = None;
                let mut global_time_selectable_enabled: Option<Option<bool>> = None;
                let mut invitees: Option<
                    Vec<crate::datadogV1::model::SharedDashboardInviteesItems>,
                > = None;
                let mut selectable_template_vars: Option<
                    Option<Vec<crate::datadogV1::model::SelectableTemplateVariableItems>>,
                > = None;
                let mut share_list: Option<Option<Vec<String>>> = None;
                let mut share_type: Option<Option<crate::datadogV1::model::DashboardShareType>> =
                    None;
                let mut status: Option<crate::datadogV1::model::SharedDashboardStatus> = None;
                let mut title: Option<String> = None;
                let mut viewing_preferences: Option<crate::datadogV1::model::ViewingPreferences> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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

                #[allow(deprecated)]
                let content = SharedDashboardUpdateRequest {
                    embeddable_domains,
                    expiration,
                    global_time,
                    global_time_selectable_enabled,
                    invitees,
                    selectable_template_vars,
                    share_list,
                    share_type,
                    status,
                    title,
                    viewing_preferences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardUpdateRequestVisitor)
    }
}
