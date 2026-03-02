// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an entity risk score
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityEntityRiskScoreAttributes {
    /// Configuration risks associated with the entity
    #[serde(rename = "configRisks")]
    pub config_risks: crate::datadogV2::model::SecurityEntityConfigRisks,
    /// Unique identifier for the entity
    #[serde(rename = "entityID")]
    pub entity_id: String,
    /// Metadata about the entity from cloud providers
    #[serde(rename = "entityMetadata")]
    pub entity_metadata: crate::datadogV2::model::SecurityEntityMetadata,
    /// Human-readable name of the entity
    #[serde(rename = "entityName")]
    pub entity_name: Option<String>,
    /// Cloud providers associated with the entity
    #[serde(rename = "entityProviders")]
    pub entity_providers: Vec<String>,
    /// Roles associated with the entity
    #[serde(rename = "entityRoles")]
    pub entity_roles: Option<Vec<String>>,
    /// Type of the entity (e.g., aws_iam_user, aws_ec2_instance)
    #[serde(rename = "entityType")]
    pub entity_type: String,
    /// Timestamp when the entity was first detected (Unix milliseconds)
    #[serde(rename = "firstDetected")]
    pub first_detected: i64,
    /// Title of the most recent signal detected for this entity
    #[serde(rename = "lastActivityTitle")]
    pub last_activity_title: String,
    /// Timestamp when the entity was last detected (Unix milliseconds)
    #[serde(rename = "lastDetected")]
    pub last_detected: i64,
    /// Current risk score for the entity
    #[serde(rename = "riskScore")]
    pub risk_score: f64,
    /// Change in risk score compared to previous period
    #[serde(rename = "riskScoreEvolution")]
    pub risk_score_evolution: f64,
    /// Severity level based on risk score
    #[serde(rename = "severity")]
    pub severity: crate::datadogV2::model::SecurityEntityRiskScoreAttributesSeverity,
    /// Number of security signals detected for this entity
    #[serde(rename = "signalsDetected")]
    pub signals_detected: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityEntityRiskScoreAttributes {
    pub fn new(
        config_risks: crate::datadogV2::model::SecurityEntityConfigRisks,
        entity_id: String,
        entity_metadata: crate::datadogV2::model::SecurityEntityMetadata,
        entity_providers: Vec<String>,
        entity_type: String,
        first_detected: i64,
        last_activity_title: String,
        last_detected: i64,
        risk_score: f64,
        risk_score_evolution: f64,
        severity: crate::datadogV2::model::SecurityEntityRiskScoreAttributesSeverity,
        signals_detected: i64,
    ) -> SecurityEntityRiskScoreAttributes {
        SecurityEntityRiskScoreAttributes {
            config_risks,
            entity_id,
            entity_metadata,
            entity_name: None,
            entity_providers,
            entity_roles: None,
            entity_type,
            first_detected,
            last_activity_title,
            last_detected,
            risk_score,
            risk_score_evolution,
            severity,
            signals_detected,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn entity_name(mut self, value: String) -> Self {
        self.entity_name = Some(value);
        self
    }

    pub fn entity_roles(mut self, value: Vec<String>) -> Self {
        self.entity_roles = Some(value);
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

impl<'de> Deserialize<'de> for SecurityEntityRiskScoreAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityEntityRiskScoreAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityEntityRiskScoreAttributesVisitor {
            type Value = SecurityEntityRiskScoreAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config_risks: Option<crate::datadogV2::model::SecurityEntityConfigRisks> =
                    None;
                let mut entity_id: Option<String> = None;
                let mut entity_metadata: Option<crate::datadogV2::model::SecurityEntityMetadata> =
                    None;
                let mut entity_name: Option<String> = None;
                let mut entity_providers: Option<Vec<String>> = None;
                let mut entity_roles: Option<Vec<String>> = None;
                let mut entity_type: Option<String> = None;
                let mut first_detected: Option<i64> = None;
                let mut last_activity_title: Option<String> = None;
                let mut last_detected: Option<i64> = None;
                let mut risk_score: Option<f64> = None;
                let mut risk_score_evolution: Option<f64> = None;
                let mut severity: Option<
                    crate::datadogV2::model::SecurityEntityRiskScoreAttributesSeverity,
                > = None;
                let mut signals_detected: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configRisks" => {
                            config_risks =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entityID" => {
                            entity_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entityMetadata" => {
                            entity_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entityName" => {
                            if v.is_null() {
                                continue;
                            }
                            entity_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entityProviders" => {
                            entity_providers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entityRoles" => {
                            if v.is_null() {
                                continue;
                            }
                            entity_roles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entityType" => {
                            entity_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "firstDetected" => {
                            first_detected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lastActivityTitle" => {
                            last_activity_title =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lastDetected" => {
                            last_detected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "riskScore" => {
                            risk_score = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "riskScoreEvolution" => {
                            risk_score_evolution =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _severity) = severity {
                                match _severity {
                                    crate::datadogV2::model::SecurityEntityRiskScoreAttributesSeverity::UnparsedObject(_severity) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "signalsDetected" => {
                            signals_detected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let config_risks =
                    config_risks.ok_or_else(|| M::Error::missing_field("config_risks"))?;
                let entity_id = entity_id.ok_or_else(|| M::Error::missing_field("entity_id"))?;
                let entity_metadata =
                    entity_metadata.ok_or_else(|| M::Error::missing_field("entity_metadata"))?;
                let entity_providers =
                    entity_providers.ok_or_else(|| M::Error::missing_field("entity_providers"))?;
                let entity_type =
                    entity_type.ok_or_else(|| M::Error::missing_field("entity_type"))?;
                let first_detected =
                    first_detected.ok_or_else(|| M::Error::missing_field("first_detected"))?;
                let last_activity_title = last_activity_title
                    .ok_or_else(|| M::Error::missing_field("last_activity_title"))?;
                let last_detected =
                    last_detected.ok_or_else(|| M::Error::missing_field("last_detected"))?;
                let risk_score = risk_score.ok_or_else(|| M::Error::missing_field("risk_score"))?;
                let risk_score_evolution = risk_score_evolution
                    .ok_or_else(|| M::Error::missing_field("risk_score_evolution"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let signals_detected =
                    signals_detected.ok_or_else(|| M::Error::missing_field("signals_detected"))?;

                let content = SecurityEntityRiskScoreAttributes {
                    config_risks,
                    entity_id,
                    entity_metadata,
                    entity_name,
                    entity_providers,
                    entity_roles,
                    entity_type,
                    first_detected,
                    last_activity_title,
                    last_detected,
                    risk_score,
                    risk_score_evolution,
                    severity,
                    signals_detected,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityEntityRiskScoreAttributesVisitor)
    }
}
