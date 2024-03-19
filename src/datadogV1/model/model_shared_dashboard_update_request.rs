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
    /// Timeframe setting for the shared dashboard.
    #[serialize_always]
    #[serde(rename = "global_time")]
    pub global_time: Option<crate::datadogV1::model::SharedDashboardUpdateRequestGlobalTime>,
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
        Option<Option<Vec<crate::datadogV1::model::SelectableTemplateVariableItems>>>,
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardUpdateRequest {
    pub fn new(
        global_time: Option<crate::datadogV1::model::SharedDashboardUpdateRequestGlobalTime>,
    ) -> SharedDashboardUpdateRequest {
        SharedDashboardUpdateRequest {
            global_time,
            global_time_selectable_enabled: None,
            selectable_template_vars: None,
            share_list: None,
            share_type: None,
            _unparsed: false,
        }
    }

    pub fn global_time_selectable_enabled(mut self, value: Option<bool>) -> Self {
        self.global_time_selectable_enabled = Some(value);
        self
    }

    pub fn selectable_template_vars(
        mut self,
        value: Option<Vec<crate::datadogV1::model::SelectableTemplateVariableItems>>,
    ) -> Self {
        self.selectable_template_vars = Some(value);
        self
    }

    pub fn share_list(mut self, value: Option<Vec<String>>) -> Self {
        self.share_list = Some(value);
        self
    }

    pub fn share_type(
        mut self,
        value: Option<crate::datadogV1::model::DashboardShareType>,
    ) -> Self {
        self.share_type = Some(value);
        self
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
                let mut global_time: Option<
                    Option<crate::datadogV1::model::SharedDashboardUpdateRequestGlobalTime>,
                > = None;
                let mut global_time_selectable_enabled: Option<Option<bool>> = None;
                let mut selectable_template_vars: Option<
                    Option<Vec<crate::datadogV1::model::SelectableTemplateVariableItems>>,
                > = None;
                let mut share_list: Option<Option<Vec<String>>> = None;
                let mut share_type: Option<Option<crate::datadogV1::model::DashboardShareType>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "global_time" => {
                            global_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_time_selectable_enabled" => {
                            global_time_selectable_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {}
                    }
                }
                let global_time =
                    global_time.ok_or_else(|| M::Error::missing_field("global_time"))?;

                let content = SharedDashboardUpdateRequest {
                    global_time,
                    global_time_selectable_enabled,
                    selectable_template_vars,
                    share_list,
                    share_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardUpdateRequestVisitor)
    }
}
