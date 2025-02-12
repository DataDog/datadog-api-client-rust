// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Wrapper object with the list of monitor IDs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CheckCanDeleteMonitorResponseData {
    /// An array of Monitor IDs that can be safely deleted.
    #[serde(rename = "ok")]
    pub ok: Option<Vec<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CheckCanDeleteMonitorResponseData {
    pub fn new() -> CheckCanDeleteMonitorResponseData {
        CheckCanDeleteMonitorResponseData {
            ok: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ok(mut self, value: Vec<i64>) -> Self {
        self.ok = Some(value);
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

impl Default for CheckCanDeleteMonitorResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CheckCanDeleteMonitorResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CheckCanDeleteMonitorResponseDataVisitor;
        impl<'a> Visitor<'a> for CheckCanDeleteMonitorResponseDataVisitor {
            type Value = CheckCanDeleteMonitorResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ok: Option<Vec<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ok" => {
                            if v.is_null() {
                                continue;
                            }
                            ok = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CheckCanDeleteMonitorResponseData {
                    ok,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CheckCanDeleteMonitorResponseDataVisitor)
    }
}
