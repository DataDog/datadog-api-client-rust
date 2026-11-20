// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeBackendRequest20270101OneOf4454622976 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "_query_options")]
    pub _query_options: Option<std::collections::BTreeMap<String, Option<crate::datadogV1_20270101::model::DashboardInlineSchema8>>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "change_type")]
    pub change_type: Option<crate::datadogV1_20270101::model::ChangeTypeOptions>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compare_to")]
    pub compare_to: Option<crate::datadogV1_20270101::model::CompareToOptions>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "increase_good")]
    pub increase_good: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order_by")]
    pub order_by: Option<crate::datadogV1_20270101::model::OrderByOptions>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order_dir")]
    pub order_dir: Option<crate::datadogV1_20270101::model::QuerySortOrder>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "q")]
    pub q: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_present")]
    pub show_present: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ChangeBackendRequest20270101OneOf4454622976 {
    pub fn new(q: String) -> ChangeBackendRequest20270101OneOf4454622976 {
        ChangeBackendRequest20270101OneOf4454622976 {
            _query_options: None,
            change_type: None,
            compare_to: None,
            increase_good: None,
            limit: None,
            order_by: None,
            order_dir: None,
            q,
            show_present: None,
            _unparsed: false,
        }
    }
    
    pub fn _query_options(mut self, value: std::collections::BTreeMap<String, Option<crate::datadogV1_20270101::model::DashboardInlineSchema8>>) -> Self {
        self._query_options = Some(value);
        self
    }
    
    pub fn change_type(mut self, value: crate::datadogV1_20270101::model::ChangeTypeOptions) -> Self {
        self.change_type = Some(value);
        self
    }
    
    pub fn compare_to(mut self, value: crate::datadogV1_20270101::model::CompareToOptions) -> Self {
        self.compare_to = Some(value);
        self
    }
    
    pub fn increase_good(mut self, value: bool) -> Self {
        self.increase_good = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn order_by(mut self, value: crate::datadogV1_20270101::model::OrderByOptions) -> Self {
        self.order_by = Some(value);
        self
    }
    
    pub fn order_dir(mut self, value: crate::datadogV1_20270101::model::QuerySortOrder) -> Self {
        self.order_dir = Some(value);
        self
    }
    
    pub fn show_present(mut self, value: bool) -> Self {
        self.show_present = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ChangeBackendRequest20270101OneOf4454622976 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeBackendRequest20270101OneOf4454622976Visitor;
        impl<'a> Visitor<'a> for ChangeBackendRequest20270101OneOf4454622976Visitor {
            type Value = ChangeBackendRequest20270101OneOf4454622976;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut _query_options: Option<std::collections::BTreeMap<String, Option<crate::datadogV1_20270101::model::DashboardInlineSchema8>>> = None;
                let mut change_type: Option<crate::datadogV1_20270101::model::ChangeTypeOptions> = None;
                let mut compare_to: Option<crate::datadogV1_20270101::model::CompareToOptions> = None;
                let mut increase_good: Option<bool> = None;
                let mut limit: Option<f64> = None;
                let mut order_by: Option<crate::datadogV1_20270101::model::OrderByOptions> = None;
                let mut order_dir: Option<crate::datadogV1_20270101::model::QuerySortOrder> = None;
                let mut q: Option<String> = None;
                let mut show_present: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "_query_options" => {
                            if v.is_null() {
                                continue;
                            }
                            _query_options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "change_type" => {
                            if v.is_null() {
                                continue;
                            }
                            change_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _change_type) = change_type {
                                match _change_type {
                                    crate::datadogV1_20270101::model::ChangeTypeOptions::UnparsedObject(_change_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "compare_to" => {
                            if v.is_null() {
                                continue;
                            }
                            compare_to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _compare_to) = compare_to {
                                match _compare_to {
                                    crate::datadogV1_20270101::model::CompareToOptions::UnparsedObject(_compare_to) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "increase_good" => {
                            if v.is_null() {
                                continue;
                            }
                            increase_good = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order_by" => {
                            if v.is_null() {
                                continue;
                            }
                            order_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order_by) = order_by {
                                match _order_by {
                                    crate::datadogV1_20270101::model::OrderByOptions::UnparsedObject(_order_by) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "order_dir" => {
                            if v.is_null() {
                                continue;
                            }
                            order_dir = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order_dir) = order_dir {
                                match _order_dir {
                                    crate::datadogV1_20270101::model::QuerySortOrder::UnparsedObject(_order_dir) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "q" => {
                            q = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_present" => {
                            if v.is_null() {
                                continue;
                            }
                            show_present = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let q = q.ok_or_else(|| M::Error::missing_field("q"))?;

                let content = ChangeBackendRequest20270101OneOf4454622976 {
                    _query_options,
                    change_type,
                    compare_to,
                    increase_good,
                    limit,
                    order_by,
                    order_dir,
                    q,
                    show_present,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeBackendRequest20270101OneOf4454622976Visitor)
    }
}
