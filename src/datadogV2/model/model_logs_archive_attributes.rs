// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes associated with the archive.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArchiveAttributes {
    /// An archive's destination.
    #[serialize_always]
    #[serde(rename = "destination")]
    pub destination: Option<crate::datadogV2::model::LogsArchiveDestination>,
    /// To store the tags in the archive, set the value "true".
    /// If it is set to "false", the tags will be deleted when the logs are sent to the archive.
    #[serde(rename = "include_tags")]
    pub include_tags: Option<bool>,
    /// The archive name.
    #[serde(rename = "name")]
    pub name: String,
    /// The archive query/filter. Logs matching this query are included in the archive.
    #[serde(rename = "query")]
    pub query: String,
    /// Maximum scan size for rehydration from this archive.
    #[serde(
        rename = "rehydration_max_scan_size_in_gb",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub rehydration_max_scan_size_in_gb: Option<Option<i64>>,
    /// An array of tags to add to rehydrated logs from an archive.
    #[serde(rename = "rehydration_tags")]
    pub rehydration_tags: Option<Vec<String>>,
    /// The state of the archive.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::LogsArchiveState>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArchiveAttributes {
    pub fn new(
        destination: Option<crate::datadogV2::model::LogsArchiveDestination>,
        name: String,
        query: String,
    ) -> LogsArchiveAttributes {
        LogsArchiveAttributes {
            destination,
            include_tags: None,
            name,
            query,
            rehydration_max_scan_size_in_gb: None,
            rehydration_tags: None,
            state: None,
            _unparsed: false,
        }
    }

    pub fn include_tags(&mut self, value: bool) -> &mut Self {
        self.include_tags = Some(value);
        self
    }

    pub fn rehydration_max_scan_size_in_gb(&mut self, value: Option<i64>) -> &mut Self {
        self.rehydration_max_scan_size_in_gb = Some(value);
        self
    }

    pub fn rehydration_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.rehydration_tags = Some(value);
        self
    }

    pub fn state(&mut self, value: crate::datadogV2::model::LogsArchiveState) -> &mut Self {
        self.state = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsArchiveAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArchiveAttributesVisitor;
        impl<'a> Visitor<'a> for LogsArchiveAttributesVisitor {
            type Value = LogsArchiveAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destination: Option<
                    Option<crate::datadogV2::model::LogsArchiveDestination>,
                > = None;
                let mut include_tags: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut rehydration_max_scan_size_in_gb: Option<Option<i64>> = None;
                let mut rehydration_tags: Option<Vec<String>> = None;
                let mut state: Option<crate::datadogV2::model::LogsArchiveState> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "destination" => {
                            destination =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _destination) = destination {
                                match _destination {
                                    Some(crate::datadogV2::model::LogsArchiveDestination::UnparsedObject(_destination)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "include_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            include_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rehydration_max_scan_size_in_gb" => {
                            rehydration_max_scan_size_in_gb =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rehydration_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            rehydration_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::LogsArchiveState::UnparsedObject(
                                        _state,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let destination =
                    destination.ok_or_else(|| M::Error::missing_field("destination"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = LogsArchiveAttributes {
                    destination,
                    include_tags,
                    name,
                    query,
                    rehydration_max_scan_size_in_gb,
                    rehydration_tags,
                    state,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArchiveAttributesVisitor)
    }
}
