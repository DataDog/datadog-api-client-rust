// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Recover sessions selected by a query. The range cannot exceed 180 days; from cannot be more than 4 years ago and to cannot be more than 1 hour in the future. Session availability depends on the configured archive retention.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRecoveryBatchCreateAttributes {
    /// Optional client estimate. Creation validates and stores a fresh server estimate instead of this value.
    #[serde(rename = "estimated_count")]
    pub estimated_count: Option<i64>,
    /// Start of the range in Unix milliseconds. Must be positive and earlier than to.
    #[serde(rename = "from")]
    pub from: i64,
    /// RUM search query. Not blank, at most 4096 bytes for batch creation and count.
    #[serde(rename = "query")]
    pub query: String,
    /// Query-based batch recovery.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV2::model::RumRecoveryBatchRequestType,
    /// End of the range in Unix milliseconds.
    #[serde(rename = "to")]
    pub to: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRecoveryBatchCreateAttributes {
    pub fn new(
        from: i64,
        query: String,
        request_type: crate::datadogV2::model::RumRecoveryBatchRequestType,
        to: i64,
    ) -> RumRecoveryBatchCreateAttributes {
        RumRecoveryBatchCreateAttributes {
            estimated_count: None,
            from,
            query,
            request_type,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn estimated_count(mut self, value: i64) -> Self {
        self.estimated_count = Some(value);
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

impl<'de> Deserialize<'de> for RumRecoveryBatchCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRecoveryBatchCreateAttributesVisitor;
        impl<'a> Visitor<'a> for RumRecoveryBatchCreateAttributesVisitor {
            type Value = RumRecoveryBatchCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut estimated_count: Option<i64> = None;
                let mut from: Option<i64> = None;
                let mut query: Option<String> = None;
                let mut request_type: Option<crate::datadogV2::model::RumRecoveryBatchRequestType> =
                    None;
                let mut to: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "estimated_count" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_type" => {
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV2::model::RumRecoveryBatchRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type =
                    request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = RumRecoveryBatchCreateAttributes {
                    estimated_count,
                    from,
                    query,
                    request_type,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRecoveryBatchCreateAttributesVisitor)
    }
}
