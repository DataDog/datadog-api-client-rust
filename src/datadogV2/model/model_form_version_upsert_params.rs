// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Parameters for upserting a form version.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormVersionUpsertParams {
    /// The entity tag for conflict detection.
    #[serde(rename = "etag")]
    pub etag: Option<String>,
    /// The match policy for upserting.
    #[serde(rename = "match_policy")]
    pub match_policy: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormVersionUpsertParams {
    pub fn new() -> FormVersionUpsertParams {
        FormVersionUpsertParams {
            etag: None,
            match_policy: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn etag(mut self, value: String) -> Self {
        self.etag = Some(value);
        self
    }

    pub fn match_policy(mut self, value: String) -> Self {
        self.match_policy = Some(value);
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

impl Default for FormVersionUpsertParams {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FormVersionUpsertParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormVersionUpsertParamsVisitor;
        impl<'a> Visitor<'a> for FormVersionUpsertParamsVisitor {
            type Value = FormVersionUpsertParams;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut etag: Option<String> = None;
                let mut match_policy: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "etag" => {
                            if v.is_null() {
                                continue;
                            }
                            etag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "match_policy" => {
                            if v.is_null() {
                                continue;
                            }
                            match_policy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FormVersionUpsertParams {
                    etag,
                    match_policy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormVersionUpsertParamsVisitor)
    }
}
