// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A step executed during a goal-based browser test turn.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultTurnStep {
    /// Storage bucket keys for artifacts produced during a step or test.
    #[serde(rename = "bucket_keys")]
    pub bucket_keys: Option<crate::datadogV2::model::SyntheticsTestResultBucketKeys>,
    /// Browser step configuration for this turn step.
    #[serde(rename = "config")]
    pub config: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultTurnStep {
    pub fn new() -> SyntheticsTestResultTurnStep {
        SyntheticsTestResultTurnStep {
            bucket_keys: None,
            config: None,
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

    pub fn config(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.config = Some(value);
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

impl Default for SyntheticsTestResultTurnStep {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultTurnStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultTurnStepVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultTurnStepVisitor {
            type Value = SyntheticsTestResultTurnStep;

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
                let mut config: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
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
                        "config" => {
                            if v.is_null() {
                                continue;
                            }
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultTurnStep {
                    bucket_keys,
                    config,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultTurnStepVisitor)
    }
}
