// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// OCSP response update timestamps.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultOCSPUpdates {
    /// Unix timestamp (ms) of the next expected OCSP update.
    #[serde(rename = "next_update")]
    pub next_update: Option<i64>,
    /// Unix timestamp (ms) of when the OCSP response was produced.
    #[serde(rename = "produced_at")]
    pub produced_at: Option<i64>,
    /// Unix timestamp (ms) of this OCSP update.
    #[serde(rename = "this_update")]
    pub this_update: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultOCSPUpdates {
    pub fn new() -> SyntheticsTestResultOCSPUpdates {
        SyntheticsTestResultOCSPUpdates {
            next_update: None,
            produced_at: None,
            this_update: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn next_update(mut self, value: i64) -> Self {
        self.next_update = Some(value);
        self
    }

    pub fn produced_at(mut self, value: i64) -> Self {
        self.produced_at = Some(value);
        self
    }

    pub fn this_update(mut self, value: i64) -> Self {
        self.this_update = Some(value);
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

impl Default for SyntheticsTestResultOCSPUpdates {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultOCSPUpdates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultOCSPUpdatesVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultOCSPUpdatesVisitor {
            type Value = SyntheticsTestResultOCSPUpdates;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut next_update: Option<i64> = None;
                let mut produced_at: Option<i64> = None;
                let mut this_update: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "next_update" => {
                            if v.is_null() {
                                continue;
                            }
                            next_update =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "produced_at" => {
                            if v.is_null() {
                                continue;
                            }
                            produced_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "this_update" => {
                            if v.is_null() {
                                continue;
                            }
                            this_update =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultOCSPUpdates {
                    next_update,
                    produced_at,
                    this_update,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultOCSPUpdatesVisitor)
    }
}
