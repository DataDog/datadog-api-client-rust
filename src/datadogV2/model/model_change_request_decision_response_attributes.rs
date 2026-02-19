// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a change request decision in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeRequestDecisionResponseAttributes {
    /// The status of a change request decision.
    #[serde(rename = "change_request_status")]
    pub change_request_status: crate::datadogV2::model::ChangeRequestDecisionStatusType,
    /// Timestamp of when the decision was made.
    #[serde(rename = "decided_at")]
    pub decided_at: chrono::DateTime<chrono::Utc>,
    /// The reason for the decision.
    #[serde(rename = "decision_reason")]
    pub decision_reason: String,
    /// Timestamp of when the decision was deleted.
    #[serde(rename = "deleted_at")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
    /// The reason for requesting the decision.
    #[serde(rename = "request_reason")]
    pub request_reason: String,
    /// Timestamp of when the decision was requested.
    #[serde(rename = "requested_at")]
    pub requested_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeRequestDecisionResponseAttributes {
    pub fn new(
        change_request_status: crate::datadogV2::model::ChangeRequestDecisionStatusType,
        decided_at: chrono::DateTime<chrono::Utc>,
        decision_reason: String,
        deleted_at: chrono::DateTime<chrono::Utc>,
        request_reason: String,
        requested_at: chrono::DateTime<chrono::Utc>,
    ) -> ChangeRequestDecisionResponseAttributes {
        ChangeRequestDecisionResponseAttributes {
            change_request_status,
            decided_at,
            decision_reason,
            deleted_at,
            request_reason,
            requested_at,
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

impl<'de> Deserialize<'de> for ChangeRequestDecisionResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeRequestDecisionResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ChangeRequestDecisionResponseAttributesVisitor {
            type Value = ChangeRequestDecisionResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_request_status: Option<
                    crate::datadogV2::model::ChangeRequestDecisionStatusType,
                > = None;
                let mut decided_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut decision_reason: Option<String> = None;
                let mut deleted_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut request_reason: Option<String> = None;
                let mut requested_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_request_status" => {
                            change_request_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _change_request_status) = change_request_status {
                                match _change_request_status {
                                    crate::datadogV2::model::ChangeRequestDecisionStatusType::UnparsedObject(_change_request_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "decided_at" => {
                            decided_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "decision_reason" => {
                            decision_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_reason" => {
                            request_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requested_at" => {
                            requested_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let change_request_status = change_request_status
                    .ok_or_else(|| M::Error::missing_field("change_request_status"))?;
                let decided_at = decided_at.ok_or_else(|| M::Error::missing_field("decided_at"))?;
                let decision_reason =
                    decision_reason.ok_or_else(|| M::Error::missing_field("decision_reason"))?;
                let deleted_at = deleted_at.ok_or_else(|| M::Error::missing_field("deleted_at"))?;
                let request_reason =
                    request_reason.ok_or_else(|| M::Error::missing_field("request_reason"))?;
                let requested_at =
                    requested_at.ok_or_else(|| M::Error::missing_field("requested_at"))?;

                let content = ChangeRequestDecisionResponseAttributes {
                    change_request_status,
                    decided_at,
                    decision_reason,
                    deleted_at,
                    request_reason,
                    requested_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeRequestDecisionResponseAttributesVisitor)
    }
}
