// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Resolution metadata for an anomaly that has been dismissed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostAnomalyDismissal {
    /// Reason the anomaly was dismissed.
    #[serde(rename = "cause")]
    pub cause: String,
    /// Unique identifier of the dismissal record.
    #[serde(rename = "dismissal_id")]
    pub dismissal_id: String,
    /// Optional message explaining the dismissal.
    #[serde(rename = "message")]
    pub message: String,
    /// Timestamp of the last dismissal update in Unix milliseconds.
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    /// Identifier of the user that last updated the dismissal.
    #[serde(rename = "updated_by")]
    pub updated_by: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostAnomalyDismissal {
    pub fn new(
        cause: String,
        dismissal_id: String,
        message: String,
        updated_at: i64,
        updated_by: String,
    ) -> CostAnomalyDismissal {
        CostAnomalyDismissal {
            cause,
            dismissal_id,
            message,
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

impl<'de> Deserialize<'de> for CostAnomalyDismissal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostAnomalyDismissalVisitor;
        impl<'a> Visitor<'a> for CostAnomalyDismissalVisitor {
            type Value = CostAnomalyDismissal;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cause: Option<String> = None;
                let mut dismissal_id: Option<String> = None;
                let mut message: Option<String> = None;
                let mut updated_at: Option<i64> = None;
                let mut updated_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cause" => {
                            cause = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dismissal_id" => {
                            dismissal_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                let cause = cause.ok_or_else(|| M::Error::missing_field("cause"))?;
                let dismissal_id =
                    dismissal_id.ok_or_else(|| M::Error::missing_field("dismissal_id"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by = updated_by.ok_or_else(|| M::Error::missing_field("updated_by"))?;

                let content = CostAnomalyDismissal {
                    cause,
                    dismissal_id,
                    message,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostAnomalyDismissalVisitor)
    }
}
