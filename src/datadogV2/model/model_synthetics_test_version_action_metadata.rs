// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing metadata about a change action.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestVersionActionMetadata {
    /// The value of the property after the change.
    #[serde(rename = "after_value")]
    pub after_value: Option<serde_json::Value>,
    /// The value of the property before the change.
    #[serde(rename = "before_value")]
    pub before_value: Option<serde_json::Value>,
    /// List of diff patches for text changes.
    #[serde(
        rename = "diff_patches",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub diff_patches:
        Option<Option<Vec<crate::datadogV2::model::SyntheticsTestVersionDiffPatches>>>,
    /// The dot-separated path of the property that was changed.
    #[serde(rename = "property_path")]
    pub property_path: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestVersionActionMetadata {
    pub fn new() -> SyntheticsTestVersionActionMetadata {
        SyntheticsTestVersionActionMetadata {
            after_value: None,
            before_value: None,
            diff_patches: None,
            property_path: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn after_value(mut self, value: serde_json::Value) -> Self {
        self.after_value = Some(value);
        self
    }

    pub fn before_value(mut self, value: serde_json::Value) -> Self {
        self.before_value = Some(value);
        self
    }

    pub fn diff_patches(
        mut self,
        value: Option<Vec<crate::datadogV2::model::SyntheticsTestVersionDiffPatches>>,
    ) -> Self {
        self.diff_patches = Some(value);
        self
    }

    pub fn property_path(mut self, value: String) -> Self {
        self.property_path = Some(value);
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

impl Default for SyntheticsTestVersionActionMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestVersionActionMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestVersionActionMetadataVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestVersionActionMetadataVisitor {
            type Value = SyntheticsTestVersionActionMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut after_value: Option<serde_json::Value> = None;
                let mut before_value: Option<serde_json::Value> = None;
                let mut diff_patches: Option<
                    Option<Vec<crate::datadogV2::model::SyntheticsTestVersionDiffPatches>>,
                > = None;
                let mut property_path: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "after_value" => {
                            if v.is_null() {
                                continue;
                            }
                            after_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "before_value" => {
                            if v.is_null() {
                                continue;
                            }
                            before_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "diff_patches" => {
                            diff_patches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "property_path" => {
                            if v.is_null() {
                                continue;
                            }
                            property_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestVersionActionMetadata {
                    after_value,
                    before_value,
                    diff_patches,
                    property_path,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestVersionActionMetadataVisitor)
    }
}
