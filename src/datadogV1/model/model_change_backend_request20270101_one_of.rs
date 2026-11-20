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
pub struct ChangeBackendRequest20270101OneOf {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "change_type")]
    pub change_type: Option<crate::datadogV1::model::ChangeTypeOptions>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "increase_good")]
    pub increase_good: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order_by")]
    pub order_by: Option<crate::datadogV1::model::OrderByOptions>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order_dir")]
    pub order_dir: Option<crate::datadogV1::model::QuerySortOrder>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::ChangeBackendRequestAnyOfAnyOfItemResponseFormat,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_present")]
    pub show_present: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::ScalarFormulaSort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ChangeBackendRequest20270101OneOf {
    pub fn new(queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>, response_format: crate::datadogV1::model::ChangeBackendRequestAnyOfAnyOfItemResponseFormat) -> ChangeBackendRequest20270101OneOf {
        ChangeBackendRequest20270101OneOf {
            change_type: None,
            formulas: None,
            increase_good: None,
            limit: None,
            order_by: None,
            order_dir: None,
            queries,
            response_format,
            show_present: None,
            sort: None,
            _unparsed: false,
        }
    }
    
    pub fn change_type(mut self, value: crate::datadogV1::model::ChangeTypeOptions) -> Self {
        self.change_type = Some(value);
        self
    }
    
    pub fn formulas(mut self, value: Vec<crate::datadogV1::model::WidgetFormula>) -> Self {
        self.formulas = Some(value);
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
    
    pub fn order_by(mut self, value: crate::datadogV1::model::OrderByOptions) -> Self {
        self.order_by = Some(value);
        self
    }
    
    pub fn order_dir(mut self, value: crate::datadogV1::model::QuerySortOrder) -> Self {
        self.order_dir = Some(value);
        self
    }
    
    pub fn show_present(mut self, value: bool) -> Self {
        self.show_present = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1::model::ScalarFormulaSort) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ChangeBackendRequest20270101OneOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeBackendRequest20270101OneOfVisitor;
        impl<'a> Visitor<'a> for ChangeBackendRequest20270101OneOfVisitor {
            type Value = ChangeBackendRequest20270101OneOf;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_type: Option<crate::datadogV1::model::ChangeTypeOptions> = None;
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut increase_good: Option<bool> = None;
                let mut limit: Option<f64> = None;
                let mut order_by: Option<crate::datadogV1::model::OrderByOptions> = None;
                let mut order_dir: Option<crate::datadogV1::model::QuerySortOrder> = None;
                let mut queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>> = None;
                let mut response_format: Option<crate::datadogV1::model::ChangeBackendRequestAnyOfAnyOfItemResponseFormat> = None;
                let mut show_present: Option<bool> = None;
                let mut sort: Option<crate::datadogV1::model::ScalarFormulaSort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_type" => {
                            if v.is_null() {
                                continue;
                            }
                            change_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _change_type) = change_type {
                                match _change_type {
                                    crate::datadogV1::model::ChangeTypeOptions::UnparsedObject(_change_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::OrderByOptions::UnparsedObject(_order_by) => {
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
                                    crate::datadogV1::model::QuerySortOrder::UnparsedObject(_order_dir) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "response_format" => {
                            response_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1::model::ChangeBackendRequestAnyOfAnyOfItemResponseFormat::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "show_present" => {
                            if v.is_null() {
                                continue;
                            }
                            show_present = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = ChangeBackendRequest20270101OneOf {
                    change_type,
                    formulas,
                    increase_good,
                    limit,
                    order_by,
                    order_dir,
                    queries,
                    response_format,
                    show_present,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeBackendRequest20270101OneOfVisitor)
    }
}
