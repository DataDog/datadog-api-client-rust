// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information on your dashboard lists.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardListListResponse {
    /// List of all your dashboard lists.
    #[serde(rename = "dashboard_lists")]
    pub dashboard_lists: Option<Vec<crate::datadogV1::model::DashboardList>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardListListResponse {
    pub fn new() -> DashboardListListResponse {
        DashboardListListResponse {
            dashboard_lists: None,
            _unparsed: false,
        }
    }

    pub fn dashboard_lists(mut self, value: Vec<crate::datadogV1::model::DashboardList>) -> Self {
        self.dashboard_lists = Some(value);
        self
    }
}

impl Default for DashboardListListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardListListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardListListResponseVisitor;
        impl<'a> Visitor<'a> for DashboardListListResponseVisitor {
            type Value = DashboardListListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dashboard_lists: Option<Vec<crate::datadogV1::model::DashboardList>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dashboard_lists" => {
                            if v.is_null() {
                                continue;
                            }
                            dashboard_lists =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DashboardListListResponse {
                    dashboard_lists,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardListListResponseVisitor)
    }
}
