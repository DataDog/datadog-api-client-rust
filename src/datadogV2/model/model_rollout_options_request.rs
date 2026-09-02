// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Rollout options request payload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RolloutOptionsRequest {
    /// Whether the schedule should begin automatically. Deprecated in favor of
    /// `scheduled_start`, which takes precedence when both are set.
    #[deprecated]
    #[serde(
        rename = "autostart",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub autostart: Option<Option<bool>>,
    /// Controls when the schedule starts. Supersedes `autostart`. One of:
    ///
    /// - `none`: create the schedule without starting it.
    /// - `now`: start the schedule immediately.
    /// - `relative:<duration>`: start after a duration (for example `relative:2h`).
    /// - `absolute:<RFC3339 timestamp>`: start at a specific time (for example `absolute:2025-06-13T12:00:00Z`).
    ///
    /// An `absolute` timestamp in the past or present is treated as `now`. A future start time
    /// is not supported for allocations linked to a standard experiment.
    #[serde(rename = "scheduled_start")]
    pub scheduled_start: Option<String>,
    /// Interval in milliseconds for uniform interval strategies.
    #[serde(rename = "selection_interval_ms")]
    pub selection_interval_ms: Option<i64>,
    /// The progression strategy used by a progressive rollout.
    #[serde(rename = "strategy")]
    pub strategy: crate::datadogV2::model::RolloutStrategy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RolloutOptionsRequest {
    pub fn new(strategy: crate::datadogV2::model::RolloutStrategy) -> RolloutOptionsRequest {
        #[allow(deprecated)]
        RolloutOptionsRequest {
            autostart: None,
            scheduled_start: None,
            selection_interval_ms: None,
            strategy,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn autostart(mut self, value: Option<bool>) -> Self {
        self.autostart = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn scheduled_start(mut self, value: String) -> Self {
        self.scheduled_start = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn selection_interval_ms(mut self, value: i64) -> Self {
        self.selection_interval_ms = Some(value);
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

impl<'de> Deserialize<'de> for RolloutOptionsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RolloutOptionsRequestVisitor;
        impl<'a> Visitor<'a> for RolloutOptionsRequestVisitor {
            type Value = RolloutOptionsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut autostart: Option<Option<bool>> = None;
                let mut scheduled_start: Option<String> = None;
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
                        "scheduled_start" => {
                            if v.is_null() {
                                continue;
                            }
                            scheduled_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selection_interval_ms" => {
                            if v.is_null() {
                                continue;
                            }
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
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;

                #[allow(deprecated)]
                let content = RolloutOptionsRequest {
                    autostart,
                    scheduled_start,
                    selection_interval_ms,
                    strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RolloutOptionsRequestVisitor)
    }
}
