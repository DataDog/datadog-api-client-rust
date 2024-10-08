// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboardInvitesMeta {
    /// Object containing the total count of invitations across all pages
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV1::model::SharedDashboardInvitesMetaPage>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardInvitesMeta {
    pub fn new() -> SharedDashboardInvitesMeta {
        SharedDashboardInvitesMeta {
            page: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn page(mut self, value: crate::datadogV1::model::SharedDashboardInvitesMetaPage) -> Self {
        self.page = Some(value);
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

impl Default for SharedDashboardInvitesMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SharedDashboardInvitesMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardInvitesMetaVisitor;
        impl<'a> Visitor<'a> for SharedDashboardInvitesMetaVisitor {
            type Value = SharedDashboardInvitesMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page: Option<crate::datadogV1::model::SharedDashboardInvitesMetaPage> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SharedDashboardInvitesMeta {
                    page,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardInvitesMetaVisitor)
    }
}
