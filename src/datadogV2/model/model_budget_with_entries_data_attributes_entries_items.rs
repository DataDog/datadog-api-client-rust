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
pub struct BudgetWithEntriesDataAttributesEntriesItems {
    #[serde(rename = "amount")]
    pub amount: Option<f64>,
    #[serde(rename = "month")]
    pub month: Option<i64>,
    #[serde(rename = "tag_filters")]
    pub tag_filters: Option<
        Vec<crate::datadogV2::model::BudgetWithEntriesDataAttributesEntriesItemsTagFiltersItems>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BudgetWithEntriesDataAttributesEntriesItems {
    pub fn new() -> BudgetWithEntriesDataAttributesEntriesItems {
        BudgetWithEntriesDataAttributesEntriesItems {
            amount: None,
            month: None,
            tag_filters: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn tag_filters(
        mut self,
        value: Vec<
            crate::datadogV2::model::BudgetWithEntriesDataAttributesEntriesItemsTagFiltersItems,
        >,
    ) -> Self {
        self.tag_filters = Some(value);
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

impl Default for BudgetWithEntriesDataAttributesEntriesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BudgetWithEntriesDataAttributesEntriesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BudgetWithEntriesDataAttributesEntriesItemsVisitor;
        impl<'a> Visitor<'a> for BudgetWithEntriesDataAttributesEntriesItemsVisitor {
            type Value = BudgetWithEntriesDataAttributesEntriesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut amount: Option<f64> = None;
                let mut month: Option<i64> = None;
                let mut tag_filters: Option<Vec<crate::datadogV2::model::BudgetWithEntriesDataAttributesEntriesItemsTagFiltersItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "amount" => {
                            if v.is_null() {
                                continue;
                            }
                            amount = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "month" => {
                            if v.is_null() {
                                continue;
                            }
                            month = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = BudgetWithEntriesDataAttributesEntriesItems {
                    amount,
                    month,
                    tag_filters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BudgetWithEntriesDataAttributesEntriesItemsVisitor)
    }
}
