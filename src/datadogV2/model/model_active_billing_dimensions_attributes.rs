// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of active billing dimensions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActiveBillingDimensionsAttributes {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]`.
    #[serde(rename = "month")]
    pub month: Option<DateTime<Utc>>,
    /// List of active billing dimensions. Example: `[infra_host, apm_host, serverless_infra]`.
    #[serde(rename = "values")]
    pub values: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ActiveBillingDimensionsAttributes {
    pub fn new() -> ActiveBillingDimensionsAttributes {
        ActiveBillingDimensionsAttributes {
            month: None,
            values: None,
            _unparsed: false,
        }
    }

    pub fn month(mut self, value: DateTime<Utc>) -> Self {
        self.month = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
        self
    }
}

impl Default for ActiveBillingDimensionsAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ActiveBillingDimensionsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActiveBillingDimensionsAttributesVisitor;
        impl<'a> Visitor<'a> for ActiveBillingDimensionsAttributesVisitor {
            type Value = ActiveBillingDimensionsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut month: Option<DateTime<Utc>> = None;
                let mut values: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "month" => {
                            if v.is_null() {
                                continue;
                            }
                            month = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ActiveBillingDimensionsAttributes {
                    month,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ActiveBillingDimensionsAttributesVisitor)
    }
}
