// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A turn in a goal-based browser test, grouping steps and reasoning.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultTurn {
    /// Storage bucket keys for artifacts produced during a step or test.
    #[serde(rename = "bucket_keys")]
    pub bucket_keys: Option<crate::datadogV2::model::SyntheticsTestResultBucketKeys>,
    /// Name of the turn.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Agent reasoning produced for this turn.
    #[serde(rename = "reasoning")]
    pub reasoning: Option<String>,
    /// Status of the turn (for example, `passed`, `failed`).
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Steps executed during the turn.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV2::model::SyntheticsTestResultTurnStep>>,
    /// Unix timestamp (ms) of when the turn finished.
    #[serde(rename = "turn_finished_at")]
    pub turn_finished_at: Option<i64>,
    /// Unix timestamp (ms) of when the turn started.
    #[serde(rename = "turn_started_at")]
    pub turn_started_at: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultTurn {
    pub fn new() -> SyntheticsTestResultTurn {
        SyntheticsTestResultTurn {
            bucket_keys: None,
            name: None,
            reasoning: None,
            status: None,
            steps: None,
            turn_finished_at: None,
            turn_started_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bucket_keys(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultBucketKeys,
    ) -> Self {
        self.bucket_keys = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn reasoning(mut self, value: String) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn steps(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultTurnStep>,
    ) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn turn_finished_at(mut self, value: i64) -> Self {
        self.turn_finished_at = Some(value);
        self
    }

    pub fn turn_started_at(mut self, value: i64) -> Self {
        self.turn_started_at = Some(value);
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

impl Default for SyntheticsTestResultTurn {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultTurn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultTurnVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultTurnVisitor {
            type Value = SyntheticsTestResultTurn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket_keys: Option<
                    crate::datadogV2::model::SyntheticsTestResultBucketKeys,
                > = None;
                let mut name: Option<String> = None;
                let mut reasoning: Option<String> = None;
                let mut status: Option<String> = None;
                let mut steps: Option<Vec<crate::datadogV2::model::SyntheticsTestResultTurnStep>> =
                    None;
                let mut turn_finished_at: Option<i64> = None;
                let mut turn_started_at: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucket_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reasoning" => {
                            if v.is_null() {
                                continue;
                            }
                            reasoning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "turn_finished_at" => {
                            if v.is_null() {
                                continue;
                            }
                            turn_finished_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "turn_started_at" => {
                            if v.is_null() {
                                continue;
                            }
                            turn_started_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultTurn {
                    bucket_keys,
                    name,
                    reasoning,
                    status,
                    steps,
                    turn_finished_at,
                    turn_started_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultTurnVisitor)
    }
}
