// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Element locator updates produced during a step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultStepElementUpdates {
    /// Updated multi-locator definition.
    #[serde(rename = "multi_locator")]
    pub multi_locator: Option<std::collections::BTreeMap<String, String>>,
    /// Updated outer HTML of the targeted element.
    #[serde(rename = "target_outer_html")]
    pub target_outer_html: Option<String>,
    /// Version of the element locator definition.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultStepElementUpdates {
    pub fn new() -> SyntheticsTestResultStepElementUpdates {
        SyntheticsTestResultStepElementUpdates {
            multi_locator: None,
            target_outer_html: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn multi_locator(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.multi_locator = Some(value);
        self
    }

    pub fn target_outer_html(mut self, value: String) -> Self {
        self.target_outer_html = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
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

impl Default for SyntheticsTestResultStepElementUpdates {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultStepElementUpdates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultStepElementUpdatesVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultStepElementUpdatesVisitor {
            type Value = SyntheticsTestResultStepElementUpdates;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut multi_locator: Option<std::collections::BTreeMap<String, String>> = None;
                let mut target_outer_html: Option<String> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "multi_locator" => {
                            if v.is_null() {
                                continue;
                            }
                            multi_locator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_outer_html" => {
                            if v.is_null() {
                                continue;
                            }
                            target_outer_html =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultStepElementUpdates {
                    multi_locator,
                    target_outer_html,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultStepElementUpdatesVisitor)
    }
}
