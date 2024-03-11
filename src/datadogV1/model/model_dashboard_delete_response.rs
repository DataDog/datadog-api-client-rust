// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response from the delete dashboard call.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardDeleteResponse {
    /// ID of the deleted dashboard.
    #[serde(rename = "deleted_dashboard_id")]
    pub deleted_dashboard_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardDeleteResponse {
    pub fn new() -> DashboardDeleteResponse {
        DashboardDeleteResponse {
            deleted_dashboard_id: None,
            _unparsed: false,
        }
    }

    pub fn deleted_dashboard_id(&mut self, value: String) -> &mut Self {
        self.deleted_dashboard_id = Some(value);
        self
    }
}

impl Default for DashboardDeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardDeleteResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardDeleteResponseVisitor;
        impl<'a> Visitor<'a> for DashboardDeleteResponseVisitor {
            type Value = DashboardDeleteResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted_dashboard_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deleted_dashboard_id" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_dashboard_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DashboardDeleteResponse {
                    deleted_dashboard_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardDeleteResponseVisitor)
    }
}
