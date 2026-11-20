// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// The bandit being refreshed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditRefreshV2DTODataAttributes {
    /// The ID of the bandit being refreshed.
    #[serde(rename = "bandit_id")]
    pub bandit_id: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsBanditRefreshV2DTODataAttributes {
    pub fn new(bandit_id: uuid::Uuid) -> ExperimentsBanditRefreshV2DTODataAttributes {
        ExperimentsBanditRefreshV2DTODataAttributes {
            bandit_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for ExperimentsBanditRefreshV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditRefreshV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsBanditRefreshV2DTODataAttributesVisitor {
            type Value = ExperimentsBanditRefreshV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bandit_id: Option<uuid::Uuid> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bandit_id" => {
                            bandit_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let bandit_id = bandit_id.ok_or_else(|| M::Error::missing_field("bandit_id"))?;

                let content = ExperimentsBanditRefreshV2DTODataAttributes {
                    bandit_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditRefreshV2DTODataAttributesVisitor)
    }
}
