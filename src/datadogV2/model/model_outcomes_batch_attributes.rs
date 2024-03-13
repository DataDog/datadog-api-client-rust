// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The JSON:API attributes for a batched set of scorecard outcomes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutcomesBatchAttributes {
    /// Set of scorecard outcomes to update.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::datadogV2::model::OutcomesBatchRequestItem>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutcomesBatchAttributes {
    pub fn new() -> OutcomesBatchAttributes {
        OutcomesBatchAttributes {
            results: None,
            _unparsed: false,
        }
    }

    pub fn results(
        mut self,
        value: Vec<crate::datadogV2::model::OutcomesBatchRequestItem>,
    ) -> Self {
        self.results = Some(value);
        self
    }
}

impl Default for OutcomesBatchAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OutcomesBatchAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutcomesBatchAttributesVisitor;
        impl<'a> Visitor<'a> for OutcomesBatchAttributesVisitor {
            type Value = OutcomesBatchAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut results: Option<Vec<crate::datadogV2::model::OutcomesBatchRequestItem>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "results" => {
                            if v.is_null() {
                                continue;
                            }
                            results = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OutcomesBatchAttributes { results, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutcomesBatchAttributesVisitor)
    }
}
