// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Per-item status returned for a bulk subscription request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SampleLogGenerationBulkSubscriptionItemMeta {
    /// A description of the error encountered for this content pack, if the subscription could not be created.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// The HTTP status code that resulted from creating the subscription for this content pack.
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SampleLogGenerationBulkSubscriptionItemMeta {
    pub fn new(status: i32) -> SampleLogGenerationBulkSubscriptionItemMeta {
        SampleLogGenerationBulkSubscriptionItemMeta {
            error: None,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error(mut self, value: String) -> Self {
        self.error = Some(value);
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

impl<'de> Deserialize<'de> for SampleLogGenerationBulkSubscriptionItemMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SampleLogGenerationBulkSubscriptionItemMetaVisitor;
        impl<'a> Visitor<'a> for SampleLogGenerationBulkSubscriptionItemMetaVisitor {
            type Value = SampleLogGenerationBulkSubscriptionItemMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<String> = None;
                let mut status: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = SampleLogGenerationBulkSubscriptionItemMeta {
                    error,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SampleLogGenerationBulkSubscriptionItemMetaVisitor)
    }
}
