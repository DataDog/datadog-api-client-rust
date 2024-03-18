// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for Azure config pair.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AzureUCConfigPairAttributes {
    /// An Azure config.
    #[serde(rename = "configs")]
    pub configs: Vec<crate::datadogV2::model::AzureUCConfig>,
    /// The ID of the Azure config pair.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AzureUCConfigPairAttributes {
    pub fn new(
        configs: Vec<crate::datadogV2::model::AzureUCConfig>,
    ) -> AzureUCConfigPairAttributes {
        AzureUCConfigPairAttributes {
            configs,
            id: None,
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AzureUCConfigPairAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AzureUCConfigPairAttributesVisitor;
        impl<'a> Visitor<'a> for AzureUCConfigPairAttributesVisitor {
            type Value = AzureUCConfigPairAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configs: Option<Vec<crate::datadogV2::model::AzureUCConfig>> = None;
                let mut id: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configs" => {
                            configs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let configs = configs.ok_or_else(|| M::Error::missing_field("configs"))?;

                let content = AzureUCConfigPairAttributes {
                    configs,
                    id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AzureUCConfigPairAttributesVisitor)
    }
}
