// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cost data for a single budget entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BudgetWithEntriesDataAttributesEntriesItemsCosts {
    /// The actual cost for this entry. Present only when `actual=true` is requested.
    #[serde(rename = "actual", default, with = "::serde_with::rust::double_option")]
    pub actual: Option<Option<f64>>,
    /// The budgeted amount for this entry.
    #[serde(rename = "amount", default, with = "::serde_with::rust::double_option")]
    pub amount: Option<Option<f64>>,
    /// The custom forecast override for this entry. `null` when `forecast=true` is requested but no custom forecast has been set for this entry's month. A numeric value, including `0`, indicates an explicit custom forecast override. Omitted when `forecast=false` or the feature is not available for the organization.
    #[serde(
        rename = "custom_forecast",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub custom_forecast: Option<Option<f64>>,
    /// The final forecast for this entry, with any custom forecast override applied. Present only when `forecast=true` is requested.
    #[serde(
        rename = "forecast",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub forecast: Option<Option<f64>>,
    /// The out-of-the-box ML forecast for this entry, before custom overrides. Present only when `forecast=true` is requested.
    #[serde(
        rename = "ootb_forecast",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ootb_forecast: Option<Option<f64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BudgetWithEntriesDataAttributesEntriesItemsCosts {
    pub fn new() -> BudgetWithEntriesDataAttributesEntriesItemsCosts {
        BudgetWithEntriesDataAttributesEntriesItemsCosts {
            actual: None,
            amount: None,
            custom_forecast: None,
            forecast: None,
            ootb_forecast: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn actual(mut self, value: Option<f64>) -> Self {
        self.actual = Some(value);
        self
    }

    pub fn amount(mut self, value: Option<f64>) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn custom_forecast(mut self, value: Option<f64>) -> Self {
        self.custom_forecast = Some(value);
        self
    }

    pub fn forecast(mut self, value: Option<f64>) -> Self {
        self.forecast = Some(value);
        self
    }

    pub fn ootb_forecast(mut self, value: Option<f64>) -> Self {
        self.ootb_forecast = Some(value);
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

impl Default for BudgetWithEntriesDataAttributesEntriesItemsCosts {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BudgetWithEntriesDataAttributesEntriesItemsCosts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BudgetWithEntriesDataAttributesEntriesItemsCostsVisitor;
        impl<'a> Visitor<'a> for BudgetWithEntriesDataAttributesEntriesItemsCostsVisitor {
            type Value = BudgetWithEntriesDataAttributesEntriesItemsCosts;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actual: Option<Option<f64>> = None;
                let mut amount: Option<Option<f64>> = None;
                let mut custom_forecast: Option<Option<f64>> = None;
                let mut forecast: Option<Option<f64>> = None;
                let mut ootb_forecast: Option<Option<f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actual" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            actual = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "amount" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            amount = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_forecast" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            custom_forecast =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "forecast" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            forecast = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ootb_forecast" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            ootb_forecast =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = BudgetWithEntriesDataAttributesEntriesItemsCosts {
                    actual,
                    amount,
                    custom_forecast,
                    forecast,
                    ootb_forecast,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BudgetWithEntriesDataAttributesEntriesItemsCostsVisitor)
    }
}
