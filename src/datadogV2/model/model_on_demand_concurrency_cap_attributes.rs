// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// On-demand concurrency cap attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnDemandConcurrencyCapAttributes {
    /// Value of the on-demand concurrency cap.
    #[serde(rename = "on_demand_concurrency_cap")]
    pub on_demand_concurrency_cap: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnDemandConcurrencyCapAttributes {
    pub fn new() -> OnDemandConcurrencyCapAttributes {
        OnDemandConcurrencyCapAttributes {
            on_demand_concurrency_cap: None,
            _unparsed: false,
        }
    }

    pub fn on_demand_concurrency_cap(mut self, value: f64) -> Self {
        self.on_demand_concurrency_cap = Some(value);
        self
    }
}

impl Default for OnDemandConcurrencyCapAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OnDemandConcurrencyCapAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnDemandConcurrencyCapAttributesVisitor;
        impl<'a> Visitor<'a> for OnDemandConcurrencyCapAttributesVisitor {
            type Value = OnDemandConcurrencyCapAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut on_demand_concurrency_cap: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "on_demand_concurrency_cap" => {
                            if v.is_null() {
                                continue;
                            }
                            on_demand_concurrency_cap =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OnDemandConcurrencyCapAttributes {
                    on_demand_concurrency_cap,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnDemandConcurrencyCapAttributesVisitor)
    }
}
