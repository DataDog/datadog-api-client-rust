// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A STIX 2.1 indicator object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct STIXIndicatorObject {
    /// The confidence in the correctness of the indicator, from 0 through 100.
    #[serde(rename = "confidence")]
    pub confidence: Option<i32>,
    /// The time when the indicator was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// Optional external reference metadata preserved with the indicator but not interpreted during ingestion.
    #[serde(rename = "external_references")]
    pub external_references: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// The STIX indicator identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// The open vocabulary terms that categorize the indicator.
    #[serde(rename = "indicator_types")]
    pub indicator_types: Option<Vec<String>>,
    /// Optional kill chain metadata preserved with the indicator but not interpreted during ingestion.
    #[serde(rename = "kill_chain_phases")]
    pub kill_chain_phases: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Labels associated with the indicator.
    #[serde(rename = "labels")]
    pub labels: Option<Vec<String>>,
    /// The time when the indicator was last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    /// References to marking definition objects that apply to the indicator.
    #[serde(rename = "object_marking_refs")]
    pub object_marking_refs: Option<Vec<String>>,
    /// The STIX pattern that identifies the observable.
    #[serde(rename = "pattern")]
    pub pattern: String,
    /// The supported STIX pattern language.
    #[serde(rename = "pattern_type")]
    pub pattern_type: crate::datadogV2::model::STIXPatternType,
    /// Whether the indicator has been revoked.
    #[serde(rename = "revoked")]
    pub revoked: Option<bool>,
    /// The supported STIX specification version.
    #[serde(rename = "spec_version")]
    pub spec_version: crate::datadogV2::model::STIXSpecVersion,
    /// The STIX object type for an indicator.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::STIXIndicatorType,
    /// The time from which the indicator is considered valid.
    #[serde(rename = "valid_from")]
    pub valid_from: chrono::DateTime<chrono::Utc>,
    /// The time until which the indicator is considered valid.
    #[serde(rename = "valid_until")]
    pub valid_until: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl STIXIndicatorObject {
    pub fn new(
        created: chrono::DateTime<chrono::Utc>,
        id: String,
        modified: chrono::DateTime<chrono::Utc>,
        pattern: String,
        pattern_type: crate::datadogV2::model::STIXPatternType,
        spec_version: crate::datadogV2::model::STIXSpecVersion,
        type_: crate::datadogV2::model::STIXIndicatorType,
        valid_from: chrono::DateTime<chrono::Utc>,
    ) -> STIXIndicatorObject {
        STIXIndicatorObject {
            confidence: None,
            created,
            external_references: None,
            id,
            indicator_types: None,
            kill_chain_phases: None,
            labels: None,
            modified,
            object_marking_refs: None,
            pattern,
            pattern_type,
            revoked: None,
            spec_version,
            type_,
            valid_from,
            valid_until: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn confidence(mut self, value: i32) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn external_references(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.external_references = Some(value);
        self
    }

    pub fn indicator_types(mut self, value: Vec<String>) -> Self {
        self.indicator_types = Some(value);
        self
    }

    pub fn kill_chain_phases(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.kill_chain_phases = Some(value);
        self
    }

    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn object_marking_refs(mut self, value: Vec<String>) -> Self {
        self.object_marking_refs = Some(value);
        self
    }

    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    pub fn valid_until(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.valid_until = Some(value);
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

impl<'de> Deserialize<'de> for STIXIndicatorObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct STIXIndicatorObjectVisitor;
        impl<'a> Visitor<'a> for STIXIndicatorObjectVisitor {
            type Value = STIXIndicatorObject;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence: Option<i32> = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut external_references: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut id: Option<String> = None;
                let mut indicator_types: Option<Vec<String>> = None;
                let mut kill_chain_phases: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut labels: Option<Vec<String>> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut object_marking_refs: Option<Vec<String>> = None;
                let mut pattern: Option<String> = None;
                let mut pattern_type: Option<crate::datadogV2::model::STIXPatternType> = None;
                let mut revoked: Option<bool> = None;
                let mut spec_version: Option<crate::datadogV2::model::STIXSpecVersion> = None;
                let mut type_: Option<crate::datadogV2::model::STIXIndicatorType> = None;
                let mut valid_from: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut valid_until: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confidence" => {
                            if v.is_null() {
                                continue;
                            }
                            confidence = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_references" => {
                            if v.is_null() {
                                continue;
                            }
                            external_references =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indicator_types" => {
                            if v.is_null() {
                                continue;
                            }
                            indicator_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kill_chain_phases" => {
                            if v.is_null() {
                                continue;
                            }
                            kill_chain_phases =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "labels" => {
                            if v.is_null() {
                                continue;
                            }
                            labels = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "object_marking_refs" => {
                            if v.is_null() {
                                continue;
                            }
                            object_marking_refs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pattern" => {
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pattern_type" => {
                            pattern_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _pattern_type) = pattern_type {
                                match _pattern_type {
                                    crate::datadogV2::model::STIXPatternType::UnparsedObject(
                                        _pattern_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "revoked" => {
                            if v.is_null() {
                                continue;
                            }
                            revoked = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spec_version" => {
                            spec_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _spec_version) = spec_version {
                                match _spec_version {
                                    crate::datadogV2::model::STIXSpecVersion::UnparsedObject(
                                        _spec_version,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::STIXIndicatorType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "valid_from" => {
                            valid_from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid_until" => {
                            if v.is_null() {
                                continue;
                            }
                            valid_until =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let pattern = pattern.ok_or_else(|| M::Error::missing_field("pattern"))?;
                let pattern_type =
                    pattern_type.ok_or_else(|| M::Error::missing_field("pattern_type"))?;
                let spec_version =
                    spec_version.ok_or_else(|| M::Error::missing_field("spec_version"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let valid_from = valid_from.ok_or_else(|| M::Error::missing_field("valid_from"))?;

                let content = STIXIndicatorObject {
                    confidence,
                    created,
                    external_references,
                    id,
                    indicator_types,
                    kill_chain_phases,
                    labels,
                    modified,
                    object_marking_refs,
                    pattern,
                    pattern_type,
                    revoked,
                    spec_version,
                    type_,
                    valid_from,
                    valid_until,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(STIXIndicatorObjectVisitor)
    }
}
