// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the retention filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMRetentionFilterMeta {
    /// The types of changes the retention filter accepts. `complete` filters can be updated and deleted.
    /// `limited` filters are default retention filters that can't be deleted.
    #[serde(rename = "edition_mode")]
    pub edition_mode: Option<crate::datadogV2::model::RUMRetentionFilterEditionMode>,
    /// The source of the last update to the retention filter. The value is `unknown` if the request doesn't specify a source.
    #[serde(rename = "source")]
    pub source: Option<crate::datadogV2::model::RUMRetentionFilterMetaSource>,
    /// The time of the last update, as a Unix timestamp in milliseconds.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<i64>,
    /// The handle of the user who last updated the retention filter.
    #[serde(rename = "updated_by_handle")]
    pub updated_by_handle: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMRetentionFilterMeta {
    pub fn new() -> RUMRetentionFilterMeta {
        RUMRetentionFilterMeta {
            edition_mode: None,
            source: None,
            updated_at: None,
            updated_by_handle: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn edition_mode(
        mut self,
        value: crate::datadogV2::model::RUMRetentionFilterEditionMode,
    ) -> Self {
        self.edition_mode = Some(value);
        self
    }

    pub fn source(mut self, value: crate::datadogV2::model::RUMRetentionFilterMetaSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by_handle(mut self, value: String) -> Self {
        self.updated_by_handle = Some(value);
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

impl Default for RUMRetentionFilterMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RUMRetentionFilterMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMRetentionFilterMetaVisitor;
        impl<'a> Visitor<'a> for RUMRetentionFilterMetaVisitor {
            type Value = RUMRetentionFilterMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut edition_mode: Option<
                    crate::datadogV2::model::RUMRetentionFilterEditionMode,
                > = None;
                let mut source: Option<crate::datadogV2::model::RUMRetentionFilterMetaSource> =
                    None;
                let mut updated_at: Option<i64> = None;
                let mut updated_by_handle: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "edition_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            edition_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _edition_mode) = edition_mode {
                                match _edition_mode {
                                    crate::datadogV2::model::RUMRetentionFilterEditionMode::UnparsedObject(_edition_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::RUMRetentionFilterMetaSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RUMRetentionFilterMeta {
                    edition_mode,
                    source,
                    updated_at,
                    updated_by_handle,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMRetentionFilterMetaVisitor)
    }
}
