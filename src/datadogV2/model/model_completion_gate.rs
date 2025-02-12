// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Used to create conditions before running subsequent actions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CompletionGate {
    /// The definition of `CompletionCondition` object.
    #[serde(rename = "completionCondition")]
    pub completion_condition: crate::datadogV2::model::CompletionCondition,
    /// The definition of `RetryStrategy` object.
    #[serde(rename = "retryStrategy")]
    pub retry_strategy: crate::datadogV2::model::RetryStrategy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CompletionGate {
    pub fn new(
        completion_condition: crate::datadogV2::model::CompletionCondition,
        retry_strategy: crate::datadogV2::model::RetryStrategy,
    ) -> CompletionGate {
        CompletionGate {
            completion_condition,
            retry_strategy,
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

impl<'de> Deserialize<'de> for CompletionGate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CompletionGateVisitor;
        impl<'a> Visitor<'a> for CompletionGateVisitor {
            type Value = CompletionGate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut completion_condition: Option<crate::datadogV2::model::CompletionCondition> =
                    None;
                let mut retry_strategy: Option<crate::datadogV2::model::RetryStrategy> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "completionCondition" => {
                            completion_condition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retryStrategy" => {
                            retry_strategy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let completion_condition = completion_condition
                    .ok_or_else(|| M::Error::missing_field("completion_condition"))?;
                let retry_strategy =
                    retry_strategy.ok_or_else(|| M::Error::missing_field("retry_strategy"))?;

                let content = CompletionGate {
                    completion_condition,
                    retry_strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CompletionGateVisitor)
    }
}
