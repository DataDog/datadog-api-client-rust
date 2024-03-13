// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dashboards within a list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardListItems {
    /// List of dashboards in the dashboard list.
    #[serde(rename = "dashboards")]
    pub dashboards: Vec<crate::datadogV2::model::DashboardListItem>,
    /// Number of dashboards in the dashboard list.
    #[serde(rename = "total")]
    pub total: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardListItems {
    pub fn new(dashboards: Vec<crate::datadogV2::model::DashboardListItem>) -> DashboardListItems {
        DashboardListItems {
            dashboards,
            total: None,
            _unparsed: false,
        }
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DashboardListItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardListItemsVisitor;
        impl<'a> Visitor<'a> for DashboardListItemsVisitor {
            type Value = DashboardListItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dashboards: Option<Vec<crate::datadogV2::model::DashboardListItem>> = None;
                let mut total: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dashboards" => {
                            dashboards = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            if v.is_null() {
                                continue;
                            }
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let dashboards = dashboards.ok_or_else(|| M::Error::missing_field("dashboards"))?;

                let content = DashboardListItems {
                    dashboards,
                    total,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardListItemsVisitor)
    }
}
