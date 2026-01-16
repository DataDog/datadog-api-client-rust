// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Jira account
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JiraAccountAttributes {
    /// The consumer key for the Jira account
    #[serde(rename = "consumer_key")]
    pub consumer_key: String,
    /// The URL of the Jira instance
    #[serde(rename = "instance_url")]
    pub instance_url: String,
    /// Timestamp of the last webhook received
    #[serde(rename = "last_webhook_timestamp")]
    pub last_webhook_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraAccountAttributes {
    pub fn new(consumer_key: String, instance_url: String) -> JiraAccountAttributes {
        JiraAccountAttributes {
            consumer_key,
            instance_url,
            last_webhook_timestamp: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn last_webhook_timestamp(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_webhook_timestamp = Some(value);
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

impl<'de> Deserialize<'de> for JiraAccountAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraAccountAttributesVisitor;
        impl<'a> Visitor<'a> for JiraAccountAttributesVisitor {
            type Value = JiraAccountAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut consumer_key: Option<String> = None;
                let mut instance_url: Option<String> = None;
                let mut last_webhook_timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "consumer_key" => {
                            consumer_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_url" => {
                            instance_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_webhook_timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            last_webhook_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let consumer_key =
                    consumer_key.ok_or_else(|| M::Error::missing_field("consumer_key"))?;
                let instance_url =
                    instance_url.ok_or_else(|| M::Error::missing_field("instance_url"))?;

                let content = JiraAccountAttributes {
                    consumer_key,
                    instance_url,
                    last_webhook_timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraAccountAttributesVisitor)
    }
}
