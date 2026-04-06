// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Applied progression options for a progressive rollout.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RolloutOptions {
    /// Whether the schedule starts automatically.
    #[serde(rename = "autostart")]
    pub autostart: bool,
    /// Interval in milliseconds for uniform interval strategies.
    #[serde(rename = "selection_interval_ms")]
    pub selection_interval_ms: i64,
    /// The progression strategy used by a progressive rollout.
    #[serde(rename = "strategy")]
    pub strategy: crate::datadogV2::model::RolloutStrategy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RolloutOptions {
    pub fn new(
        autostart: bool,
        selection_interval_ms: i64,
        strategy: crate::datadogV2::model::RolloutStrategy,
    ) -> RolloutOptions {
        RolloutOptions {
            autostart,
            selection_interval_ms,
            strategy,
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

impl<'de> Deserialize<'de> for RolloutOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RolloutOptionsVisitor;
        impl<'a> Visitor<'a> for RolloutOptionsVisitor {
            type Value = RolloutOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut autostart: Option<bool> = None;
                let mut selection_interval_ms: Option<i64> = None;
                let mut strategy: Option<crate::datadogV2::model::RolloutStrategy> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "autostart" => {
                            autostart = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selection_interval_ms" => {
                            selection_interval_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "strategy" => {
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _strategy) = strategy {
                                match _strategy {
                                    crate::datadogV2::model::RolloutStrategy::UnparsedObject(
                                        _strategy,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let autostart = autostart.ok_or_else(|| M::Error::missing_field("autostart"))?;
                let selection_interval_ms = selection_interval_ms
                    .ok_or_else(|| M::Error::missing_field("selection_interval_ms"))?;
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;

                let content = RolloutOptions {
                    autostart,
                    selection_interval_ms,
                    strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RolloutOptionsVisitor)
    }
}
