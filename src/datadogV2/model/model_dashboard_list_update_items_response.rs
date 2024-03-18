// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing a list of updated dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardListUpdateItemsResponse {
    /// List of dashboards in the dashboard list.
    #[serde(rename = "dashboards")]
    pub dashboards: Option<Vec<crate::datadogV2::model::DashboardListItemResponse>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardListUpdateItemsResponse {
    pub fn new() -> DashboardListUpdateItemsResponse {
        DashboardListUpdateItemsResponse {
            dashboards: None,
            _unparsed: false,
        }
    }

    pub fn dashboards(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardListItemResponse>,
    ) -> Self {
        self.dashboards = Some(value);
        self
    }
}

impl Default for DashboardListUpdateItemsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardListUpdateItemsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardListUpdateItemsResponseVisitor;
        impl<'a> Visitor<'a> for DashboardListUpdateItemsResponseVisitor {
            type Value = DashboardListUpdateItemsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dashboards: Option<
                    Vec<crate::datadogV2::model::DashboardListItemResponse>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dashboards" => {
                            if v.is_null() {
                                continue;
                            }
                            dashboards = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DashboardListUpdateItemsResponse {
                    dashboards,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardListUpdateItemsResponseVisitor)
    }
}
