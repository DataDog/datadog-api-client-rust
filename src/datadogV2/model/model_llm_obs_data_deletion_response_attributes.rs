// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a submitted LLM Observability data deletion request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDataDeletionResponseAttributes {
    /// Timestamp when the deletion request was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// UUID of the user who created the deletion request.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Start of the deletion time range in milliseconds since Unix epoch.
    #[serde(rename = "from_time")]
    pub from_time: i64,
    /// ID of the organization that submitted the deletion request.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// Product name for the deletion request.
    #[serde(rename = "product")]
    pub product: String,
    /// The query string used to select data for deletion.
    #[serde(rename = "query")]
    pub query: String,
    /// End of the deletion time range in milliseconds since Unix epoch.
    #[serde(rename = "to_time")]
    pub to_time: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDataDeletionResponseAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        from_time: i64,
        org_id: i64,
        product: String,
        query: String,
        to_time: i64,
    ) -> LLMObsDataDeletionResponseAttributes {
        LLMObsDataDeletionResponseAttributes {
            created_at,
            created_by,
            from_time,
            org_id,
            product,
            query,
            to_time,
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

impl<'de> Deserialize<'de> for LLMObsDataDeletionResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDataDeletionResponseAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsDataDeletionResponseAttributesVisitor {
            type Value = LLMObsDataDeletionResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut from_time: Option<i64> = None;
                let mut org_id: Option<i64> = None;
                let mut product: Option<String> = None;
                let mut query: Option<String> = None;
                let mut to_time: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from_time" => {
                            from_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product" => {
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_time" => {
                            to_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let from_time = from_time.ok_or_else(|| M::Error::missing_field("from_time"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let to_time = to_time.ok_or_else(|| M::Error::missing_field("to_time"))?;

                let content = LLMObsDataDeletionResponseAttributes {
                    created_at,
                    created_by,
                    from_time,
                    org_id,
                    product,
                    query,
                    to_time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDataDeletionResponseAttributesVisitor)
    }
}
