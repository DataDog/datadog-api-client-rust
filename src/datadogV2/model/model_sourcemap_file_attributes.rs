// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a JavaScript source map file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SourcemapFileAttributes {
    /// The name of the minified JavaScript file.
    #[serde(rename = "file")]
    pub file: String,
    /// The Base64 VLQ encoded string that maps positions in the minified
    /// file to positions in the original source files.
    #[serde(rename = "mappings")]
    pub mappings: String,
    /// List of character counts for each line in the minified file.
    #[serde(rename = "minifiedLineLengths")]
    pub minified_line_lengths: Vec<i32>,
    /// List of symbol names referenced in the mappings.
    #[serde(rename = "names")]
    pub names: Vec<serde_json::Value>,
    /// The root path prepended to source file paths.
    #[serde(rename = "sourceRoot")]
    pub source_root: String,
    /// List of original source file paths.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// List of original source file contents corresponding to the paths in `sources`.
    #[serde(rename = "sourcesContent")]
    pub sources_content: Vec<String>,
    /// The version of the source map format (typically 3).
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SourcemapFileAttributes {
    pub fn new(
        file: String,
        mappings: String,
        minified_line_lengths: Vec<i32>,
        names: Vec<serde_json::Value>,
        source_root: String,
        sources: Vec<String>,
        sources_content: Vec<String>,
        version: i64,
    ) -> SourcemapFileAttributes {
        SourcemapFileAttributes {
            file,
            mappings,
            minified_line_lengths,
            names,
            source_root,
            sources,
            sources_content,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SourcemapFileAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SourcemapFileAttributesVisitor;
        impl<'a> Visitor<'a> for SourcemapFileAttributesVisitor {
            type Value = SourcemapFileAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut file: Option<String> = None;
                let mut mappings: Option<String> = None;
                let mut minified_line_lengths: Option<Vec<i32>> = None;
                let mut names: Option<Vec<serde_json::Value>> = None;
                let mut source_root: Option<String> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut sources_content: Option<Vec<String>> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "file" => {
                            file = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mappings" => {
                            mappings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minifiedLineLengths" => {
                            minified_line_lengths =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "names" => {
                            names = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sourceRoot" => {
                            source_root =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sourcesContent" => {
                            sources_content =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let file = file.ok_or_else(|| M::Error::missing_field("file"))?;
                let mappings = mappings.ok_or_else(|| M::Error::missing_field("mappings"))?;
                let minified_line_lengths = minified_line_lengths
                    .ok_or_else(|| M::Error::missing_field("minified_line_lengths"))?;
                let names = names.ok_or_else(|| M::Error::missing_field("names"))?;
                let source_root =
                    source_root.ok_or_else(|| M::Error::missing_field("source_root"))?;
                let sources = sources.ok_or_else(|| M::Error::missing_field("sources"))?;
                let sources_content =
                    sources_content.ok_or_else(|| M::Error::missing_field("sources_content"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = SourcemapFileAttributes {
                    file,
                    mappings,
                    minified_line_lengths,
                    names,
                    source_root,
                    sources,
                    sources_content,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SourcemapFileAttributesVisitor)
    }
}
