// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for filtering the GCP projects that Datadog monitors by project ID or by project label.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GCPProjectFilterConfig {
    /// List of filters matched against the project field set by `type`. Filters support the `*` and `?`
    /// wildcards, and a leading `!` makes a filter an exclusion. A project is skipped if it matches any
    /// exclusion filter. When inclusion filters are set, a project must also match at least one of them.
    /// For `project_id`, each filter is a project ID pattern, such as `prod-*`. You can set up to 15,000
    /// filters. For `project_label`, each filter is a label in the form `key:value`, such as `env:prod*`.
    /// You can set up to 100 filters. Keys are matched exactly, and wildcards apply only to values, so
    /// `key:*` matches any value.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<String>>,
    /// The GCP project field that the filters are matched against.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPProjectFilterConfigType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GCPProjectFilterConfig {
    pub fn new() -> GCPProjectFilterConfig {
        GCPProjectFilterConfig {
            filters: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filters(mut self, value: Vec<String>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::GCPProjectFilterConfigType) -> Self {
        self.type_ = Some(value);
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

impl Default for GCPProjectFilterConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GCPProjectFilterConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPProjectFilterConfigVisitor;
        impl<'a> Visitor<'a> for GCPProjectFilterConfigVisitor {
            type Value = GCPProjectFilterConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filters: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV2::model::GCPProjectFilterConfigType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::GCPProjectFilterConfigType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GCPProjectFilterConfig {
                    filters,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPProjectFilterConfigVisitor)
    }
}
