// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata returned alongside a list of sample log generation subscriptions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SampleLogGenerationSubscriptionsResponseMeta {
    /// The total number of subscriptions matching the request, irrespective of pagination.
    #[serde(rename = "total_subscriptions")]
    pub total_subscriptions: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SampleLogGenerationSubscriptionsResponseMeta {
    pub fn new(total_subscriptions: i32) -> SampleLogGenerationSubscriptionsResponseMeta {
        SampleLogGenerationSubscriptionsResponseMeta {
            total_subscriptions,
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

impl<'de> Deserialize<'de> for SampleLogGenerationSubscriptionsResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SampleLogGenerationSubscriptionsResponseMetaVisitor;
        impl<'a> Visitor<'a> for SampleLogGenerationSubscriptionsResponseMetaVisitor {
            type Value = SampleLogGenerationSubscriptionsResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut total_subscriptions: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "total_subscriptions" => {
                            total_subscriptions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let total_subscriptions = total_subscriptions
                    .ok_or_else(|| M::Error::missing_field("total_subscriptions"))?;

                let content = SampleLogGenerationSubscriptionsResponseMeta {
                    total_subscriptions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SampleLogGenerationSubscriptionsResponseMetaVisitor)
    }
}
