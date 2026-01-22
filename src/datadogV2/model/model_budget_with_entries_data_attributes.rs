// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BudgetWithEntriesDataAttributes {
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    #[serde(rename = "end_month")]
    pub end_month: Option<i64>,
    #[serde(rename = "entries")]
    pub entries: Option<Vec<crate::datadogV2::model::BudgetWithEntriesDataAttributesEntriesItems>>,
    #[serde(rename = "metrics_query")]
    pub metrics_query: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    #[serde(rename = "start_month")]
    pub start_month: Option<i64>,
    #[serde(rename = "total_amount")]
    pub total_amount: Option<f64>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<i64>,
    #[serde(rename = "updated_by")]
    pub updated_by: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BudgetWithEntriesDataAttributes {
    pub fn new() -> BudgetWithEntriesDataAttributes {
        BudgetWithEntriesDataAttributes {
            created_at: None,
            created_by: None,
            end_month: None,
            entries: None,
            metrics_query: None,
            name: None,
            org_id: None,
            start_month: None,
            total_amount: None,
            updated_at: None,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn end_month(mut self, value: i64) -> Self {
        self.end_month = Some(value);
        self
    }

    pub fn entries(
        mut self,
        value: Vec<crate::datadogV2::model::BudgetWithEntriesDataAttributesEntriesItems>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    pub fn metrics_query(mut self, value: String) -> Self {
        self.metrics_query = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn start_month(mut self, value: i64) -> Self {
        self.start_month = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(mut self, value: String) -> Self {
        self.updated_by = Some(value);
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

impl Default for BudgetWithEntriesDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BudgetWithEntriesDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BudgetWithEntriesDataAttributesVisitor;
        impl<'a> Visitor<'a> for BudgetWithEntriesDataAttributesVisitor {
            type Value = BudgetWithEntriesDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<i64> = None;
                let mut created_by: Option<String> = None;
                let mut end_month: Option<i64> = None;
                let mut entries: Option<
                    Vec<crate::datadogV2::model::BudgetWithEntriesDataAttributesEntriesItems>,
                > = None;
                let mut metrics_query: Option<String> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut start_month: Option<i64> = None;
                let mut total_amount: Option<f64> = None;
                let mut updated_at: Option<i64> = None;
                let mut updated_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_month" => {
                            if v.is_null() {
                                continue;
                            }
                            end_month = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entries" => {
                            if v.is_null() {
                                continue;
                            }
                            entries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics_query" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_month" => {
                            if v.is_null() {
                                continue;
                            }
                            start_month =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_amount" => {
                            if v.is_null() {
                                continue;
                            }
                            total_amount =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = BudgetWithEntriesDataAttributes {
                    created_at,
                    created_by,
                    end_month,
                    entries,
                    metrics_query,
                    name,
                    org_id,
                    start_month,
                    total_amount,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BudgetWithEntriesDataAttributesVisitor)
    }
}
