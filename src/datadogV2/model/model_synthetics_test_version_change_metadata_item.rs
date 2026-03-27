// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a single change within a version.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestVersionChangeMetadataItem {
    /// The action that was performed (for example, `updated` or `created`).
    #[serde(rename = "action")]
    pub action: Option<String>,
    /// Object containing metadata about a change action.
    #[serde(rename = "action_metadata")]
    pub action_metadata: Option<crate::datadogV2::model::SyntheticsTestVersionActionMetadata>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestVersionChangeMetadataItem {
    pub fn new() -> SyntheticsTestVersionChangeMetadataItem {
        SyntheticsTestVersionChangeMetadataItem {
            action: None,
            action_metadata: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn action(mut self, value: String) -> Self {
        self.action = Some(value);
        self
    }

    pub fn action_metadata(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestVersionActionMetadata,
    ) -> Self {
        self.action_metadata = Some(value);
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

impl Default for SyntheticsTestVersionChangeMetadataItem {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestVersionChangeMetadataItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestVersionChangeMetadataItemVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestVersionChangeMetadataItemVisitor {
            type Value = SyntheticsTestVersionChangeMetadataItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<String> = None;
                let mut action_metadata: Option<
                    crate::datadogV2::model::SyntheticsTestVersionActionMetadata,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            if v.is_null() {
                                continue;
                            }
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "action_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            action_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestVersionChangeMetadataItem {
                    action,
                    action_metadata,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestVersionChangeMetadataItemVisitor)
    }
}
