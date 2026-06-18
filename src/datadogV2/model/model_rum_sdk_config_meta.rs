// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata associated with a RUM SDK configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSdkConfigMeta {
    /// The timestamp of the last update to this configuration.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// The handle of the user who last updated this configuration.
    #[serde(rename = "updated_by")]
    pub updated_by: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSdkConfigMeta {
    pub fn new(updated_at: chrono::DateTime<chrono::Utc>, updated_by: String) -> RumSdkConfigMeta {
        RumSdkConfigMeta {
            updated_at,
            updated_by,
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

impl<'de> Deserialize<'de> for RumSdkConfigMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSdkConfigMetaVisitor;
        impl<'a> Visitor<'a> for RumSdkConfigMetaVisitor {
            type Value = RumSdkConfigMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by = updated_by.ok_or_else(|| M::Error::missing_field("updated_by"))?;

                let content = RumSdkConfigMeta {
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSdkConfigMetaVisitor)
    }
}
