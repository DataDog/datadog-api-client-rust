// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability dataset draft state.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetDraftStateDataAttributes {
    /// Timestamp when the dataset draft session started.
    #[serde(rename = "drafting_since")]
    pub drafting_since: chrono::DateTime<chrono::Utc>,
    /// User information associated with a dataset draft state.
    #[serde(rename = "user")]
    pub user: crate::datadogV2::model::LLMObsDatasetDraftStateUser,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetDraftStateDataAttributes {
    pub fn new(
        drafting_since: chrono::DateTime<chrono::Utc>,
        user: crate::datadogV2::model::LLMObsDatasetDraftStateUser,
    ) -> LLMObsDatasetDraftStateDataAttributes {
        LLMObsDatasetDraftStateDataAttributes {
            drafting_since,
            user,
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

impl<'de> Deserialize<'de> for LLMObsDatasetDraftStateDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetDraftStateDataAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetDraftStateDataAttributesVisitor {
            type Value = LLMObsDatasetDraftStateDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut drafting_since: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut user: Option<crate::datadogV2::model::LLMObsDatasetDraftStateUser> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "drafting_since" => {
                            drafting_since =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user" => {
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let drafting_since =
                    drafting_since.ok_or_else(|| M::Error::missing_field("drafting_since"))?;
                let user = user.ok_or_else(|| M::Error::missing_field("user"))?;

                let content = LLMObsDatasetDraftStateDataAttributes {
                    drafting_since,
                    user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetDraftStateDataAttributesVisitor)
    }
}
