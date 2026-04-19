// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about a sub-test played from a parent browser test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultSubTest {
    /// Identifier of the sub-test.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Index of the browser tab playing the sub-test.
    #[serde(rename = "playing_tab")]
    pub playing_tab: Option<i64>,
    /// RUM application context associated with a step or sub-test.
    #[serde(rename = "rum_context")]
    pub rum_context: Option<crate::datadogV2::model::SyntheticsTestResultRumContext>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultSubTest {
    pub fn new() -> SyntheticsTestResultSubTest {
        SyntheticsTestResultSubTest {
            id: None,
            playing_tab: None,
            rum_context: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn playing_tab(mut self, value: i64) -> Self {
        self.playing_tab = Some(value);
        self
    }

    pub fn rum_context(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultRumContext,
    ) -> Self {
        self.rum_context = Some(value);
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

impl Default for SyntheticsTestResultSubTest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultSubTest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultSubTestVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultSubTestVisitor {
            type Value = SyntheticsTestResultSubTest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut playing_tab: Option<i64> = None;
                let mut rum_context: Option<
                    crate::datadogV2::model::SyntheticsTestResultRumContext,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "playing_tab" => {
                            if v.is_null() {
                                continue;
                            }
                            playing_tab =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_context" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_context =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultSubTest {
                    id,
                    playing_tab,
                    rum_context,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultSubTestVisitor)
    }
}
