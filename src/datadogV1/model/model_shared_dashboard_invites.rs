// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Invitations data and metadata that exists for a shared dashboard returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboardInvites {
    /// An object or list of objects containing the information for an invitation to a shared dashboard.
    #[serde(rename = "data")]
    pub data: crate::datadogV1::model::SharedDashboardInvitesData,
    /// Pagination metadata returned by the API.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV1::model::SharedDashboardInvitesMeta>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardInvites {
    pub fn new(
        data: crate::datadogV1::model::SharedDashboardInvitesData,
    ) -> SharedDashboardInvites {
        SharedDashboardInvites {
            data,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV1::model::SharedDashboardInvitesMeta) -> Self {
        self.meta = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SharedDashboardInvites {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardInvitesVisitor;
        impl<'a> Visitor<'a> for SharedDashboardInvitesVisitor {
            type Value = SharedDashboardInvites;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV1::model::SharedDashboardInvitesData> = None;
                let mut meta: Option<crate::datadogV1::model::SharedDashboardInvitesMeta> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data) = data {
                                match _data {
                                    crate::datadogV1::model::SharedDashboardInvitesData::UnparsedObject(_data) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = SharedDashboardInvites {
                    data,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardInvitesVisitor)
    }
}
