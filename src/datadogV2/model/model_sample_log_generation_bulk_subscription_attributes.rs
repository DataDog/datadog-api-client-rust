// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes for creating sample log generation subscriptions for multiple content packs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SampleLogGenerationBulkSubscriptionAttributes {
    /// The identifiers of the Cloud SIEM content packs to subscribe to. At most five content packs can be requested in a single call.
    #[serde(rename = "content_pack_ids")]
    pub content_pack_ids: Vec<String>,
    /// How long the subscription should remain active before expiring.
    #[serde(rename = "duration")]
    pub duration: Option<crate::datadogV2::model::SampleLogGenerationDuration>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SampleLogGenerationBulkSubscriptionAttributes {
    pub fn new(content_pack_ids: Vec<String>) -> SampleLogGenerationBulkSubscriptionAttributes {
        SampleLogGenerationBulkSubscriptionAttributes {
            content_pack_ids,
            duration: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn duration(mut self, value: crate::datadogV2::model::SampleLogGenerationDuration) -> Self {
        self.duration = Some(value);
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

impl<'de> Deserialize<'de> for SampleLogGenerationBulkSubscriptionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SampleLogGenerationBulkSubscriptionAttributesVisitor;
        impl<'a> Visitor<'a> for SampleLogGenerationBulkSubscriptionAttributesVisitor {
            type Value = SampleLogGenerationBulkSubscriptionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content_pack_ids: Option<Vec<String>> = None;
                let mut duration: Option<crate::datadogV2::model::SampleLogGenerationDuration> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content_pack_ids" => {
                            content_pack_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _duration) = duration {
                                match _duration {
                                    crate::datadogV2::model::SampleLogGenerationDuration::UnparsedObject(_duration) => {
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
                let content_pack_ids =
                    content_pack_ids.ok_or_else(|| M::Error::missing_field("content_pack_ids"))?;

                let content = SampleLogGenerationBulkSubscriptionAttributes {
                    content_pack_ids,
                    duration,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SampleLogGenerationBulkSubscriptionAttributesVisitor)
    }
}
