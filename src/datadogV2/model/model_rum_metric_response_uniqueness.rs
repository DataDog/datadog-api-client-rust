// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The rule to count updatable events. Is only set if `event_type` is `session` or `view`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumMetricResponseUniqueness {
    /// When to count updatable events. `match` when the event is first seen, or `end` when the event is complete.
    #[serde(rename = "when")]
    pub when: Option<crate::datadogV2::model::RumMetricUniquenessWhen>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumMetricResponseUniqueness {
    pub fn new() -> RumMetricResponseUniqueness {
        RumMetricResponseUniqueness {
            when: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn when(mut self, value: crate::datadogV2::model::RumMetricUniquenessWhen) -> Self {
        self.when = Some(value);
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

impl Default for RumMetricResponseUniqueness {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumMetricResponseUniqueness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumMetricResponseUniquenessVisitor;
        impl<'a> Visitor<'a> for RumMetricResponseUniquenessVisitor {
            type Value = RumMetricResponseUniqueness;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut when: Option<crate::datadogV2::model::RumMetricUniquenessWhen> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "when" => {
                            if v.is_null() {
                                continue;
                            }
                            when = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _when) = when {
                                match _when {
                                    crate::datadogV2::model::RumMetricUniquenessWhen::UnparsedObject(_when) => {
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

                let content = RumMetricResponseUniqueness {
                    when,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumMetricResponseUniquenessVisitor)
    }
}
