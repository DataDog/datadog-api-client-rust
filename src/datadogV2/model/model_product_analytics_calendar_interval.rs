// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A calendar-aligned bucket definition, such as "every 1 week starting on Monday".
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsCalendarInterval {
    /// Where each bucket starts within the calendar unit. Use an hour for `day` (for example `1am` or `14`),
    /// a day name for `week` (for example `monday`), or an ordinal for `month` (for example `1st`).
    #[serde(rename = "alignment")]
    pub alignment: Option<String>,
    /// Number of calendar units per bucket.
    #[serde(rename = "quantity")]
    pub quantity: Option<i64>,
    /// Timezone used to align the buckets.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// Calendar unit used to bucket cohorts.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ProductAnalyticsCalendarIntervalType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsCalendarInterval {
    pub fn new(
        type_: crate::datadogV2::model::ProductAnalyticsCalendarIntervalType,
    ) -> ProductAnalyticsCalendarInterval {
        ProductAnalyticsCalendarInterval {
            alignment: None,
            quantity: None,
            timezone: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn alignment(mut self, value: String) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsCalendarInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsCalendarIntervalVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsCalendarIntervalVisitor {
            type Value = ProductAnalyticsCalendarInterval;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alignment: Option<String> = None;
                let mut quantity: Option<i64> = None;
                let mut timezone: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ProductAnalyticsCalendarIntervalType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alignment" => {
                            if v.is_null() {
                                continue;
                            }
                            alignment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "quantity" => {
                            if v.is_null() {
                                continue;
                            }
                            quantity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ProductAnalyticsCalendarIntervalType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ProductAnalyticsCalendarInterval {
                    alignment,
                    quantity,
                    timezone,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsCalendarIntervalVisitor)
    }
}
