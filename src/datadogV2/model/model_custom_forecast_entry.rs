// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A monthly entry of a custom budget forecast.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomForecastEntry {
    /// Forecast amount for the month.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Month the custom forecast entry applies to, in `YYYYMM` format.
    #[serde(rename = "month")]
    pub month: i64,
    /// Tag filters that scope this custom forecast entry to specific resources.
    #[serde(rename = "tag_filters")]
    pub tag_filters: Vec<crate::datadogV2::model::CustomForecastEntryTagFilter>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomForecastEntry {
    pub fn new(
        amount: f64,
        month: i64,
        tag_filters: Vec<crate::datadogV2::model::CustomForecastEntryTagFilter>,
    ) -> CustomForecastEntry {
        CustomForecastEntry {
            amount,
            month,
            tag_filters,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CustomForecastEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomForecastEntryVisitor;
        impl<'a> Visitor<'a> for CustomForecastEntryVisitor {
            type Value = CustomForecastEntry;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut amount: Option<f64> = None;
                let mut month: Option<i64> = None;
                let mut tag_filters: Option<
                    Vec<crate::datadogV2::model::CustomForecastEntryTagFilter>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "amount" => {
                            amount = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "month" => {
                            month = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_filters" => {
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
                let amount = amount.ok_or_else(|| M::Error::missing_field("amount"))?;
                let month = month.ok_or_else(|| M::Error::missing_field("month"))?;
                let tag_filters =
                    tag_filters.ok_or_else(|| M::Error::missing_field("tag_filters"))?;

                let content = CustomForecastEntry {
                    amount,
                    month,
                    tag_filters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomForecastEntryVisitor)
    }
}
