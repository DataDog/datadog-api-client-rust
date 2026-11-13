// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The policy the bandit serves. Null until the bandit starts.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditPolicyV2 {
    /// When the policy started being served.
    #[serde(rename = "applied_at")]
    pub applied_at: chrono::DateTime<chrono::Utc>,
    /// The configuration version the policy was made for.
    #[serde(rename = "config_version")]
    pub config_version: i64,
    /// The policy ID. Each exposure logs it as `bandit.policy_id`.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// WARMUP serves the arms equally. ADAPTIVE serves the initial split or a split from a refresh. FALLBACK serves one variant after the bandit ends.
    #[serde(rename = "kind")]
    pub kind: crate::datadogV2::model::ExperimentsBanditPolicyKind,
    /// Why the policy was applied.
    #[serde(rename = "reason")]
    pub reason: crate::datadogV2::model::ExperimentsBanditPolicyReason,
    /// Each variant's share of traffic. The probabilities add up to 1.
    #[serde(rename = "variants")]
    pub variants: Vec<crate::datadogV2::model::ExperimentsBanditPolicyVariantV2>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditPolicyV2 {
    pub fn new(
        applied_at: chrono::DateTime<chrono::Utc>,
        config_version: i64,
        id: uuid::Uuid,
        kind: crate::datadogV2::model::ExperimentsBanditPolicyKind,
        reason: crate::datadogV2::model::ExperimentsBanditPolicyReason,
        variants: Vec<crate::datadogV2::model::ExperimentsBanditPolicyVariantV2>,
    ) -> ExperimentsBanditPolicyV2 {
        ExperimentsBanditPolicyV2 {
            applied_at,
            config_version,
            id,
            kind,
            reason,
            variants,
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

impl<'de> Deserialize<'de> for ExperimentsBanditPolicyV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditPolicyV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditPolicyV2Visitor {
            type Value = ExperimentsBanditPolicyV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut applied_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut config_version: Option<i64> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut kind: Option<crate::datadogV2::model::ExperimentsBanditPolicyKind> = None;
                let mut reason: Option<crate::datadogV2::model::ExperimentsBanditPolicyReason> =
                    None;
                let mut variants: Option<
                    Vec<crate::datadogV2::model::ExperimentsBanditPolicyVariantV2>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "applied_at" => {
                            applied_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_version" => {
                            config_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kind" => {
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _kind) = kind {
                                match _kind {
                                    crate::datadogV2::model::ExperimentsBanditPolicyKind::UnparsedObject(_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "reason" => {
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reason) = reason {
                                match _reason {
                                    crate::datadogV2::model::ExperimentsBanditPolicyReason::UnparsedObject(_reason) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "variants" => {
                            variants = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let applied_at = applied_at.ok_or_else(|| M::Error::missing_field("applied_at"))?;
                let config_version =
                    config_version.ok_or_else(|| M::Error::missing_field("config_version"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let kind = kind.ok_or_else(|| M::Error::missing_field("kind"))?;
                let reason = reason.ok_or_else(|| M::Error::missing_field("reason"))?;
                let variants = variants.ok_or_else(|| M::Error::missing_field("variants"))?;

                let content = ExperimentsBanditPolicyV2 {
                    applied_at,
                    config_version,
                    id,
                    kind,
                    reason,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditPolicyV2Visitor)
    }
}
