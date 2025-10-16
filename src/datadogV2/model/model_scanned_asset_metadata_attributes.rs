// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a scanned asset metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScannedAssetMetadataAttributes {
    /// The asset of a scanned asset metadata.
    #[serde(rename = "asset")]
    pub asset: crate::datadogV2::model::ScannedAssetMetadataAsset,
    /// The timestamp when the scan of the asset was performed for the first time.
    #[serde(rename = "first_success_timestamp")]
    pub first_success_timestamp: String,
    /// Metadata for the last successful scan of an asset.
    #[serde(rename = "last_success")]
    pub last_success: crate::datadogV2::model::ScannedAssetMetadataLastSuccess,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScannedAssetMetadataAttributes {
    pub fn new(
        asset: crate::datadogV2::model::ScannedAssetMetadataAsset,
        first_success_timestamp: String,
        last_success: crate::datadogV2::model::ScannedAssetMetadataLastSuccess,
    ) -> ScannedAssetMetadataAttributes {
        ScannedAssetMetadataAttributes {
            asset,
            first_success_timestamp,
            last_success,
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

impl<'de> Deserialize<'de> for ScannedAssetMetadataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScannedAssetMetadataAttributesVisitor;
        impl<'a> Visitor<'a> for ScannedAssetMetadataAttributesVisitor {
            type Value = ScannedAssetMetadataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut asset: Option<crate::datadogV2::model::ScannedAssetMetadataAsset> = None;
                let mut first_success_timestamp: Option<String> = None;
                let mut last_success: Option<
                    crate::datadogV2::model::ScannedAssetMetadataLastSuccess,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "asset" => {
                            asset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_success_timestamp" => {
                            first_success_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_success" => {
                            last_success =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let asset = asset.ok_or_else(|| M::Error::missing_field("asset"))?;
                let first_success_timestamp = first_success_timestamp
                    .ok_or_else(|| M::Error::missing_field("first_success_timestamp"))?;
                let last_success =
                    last_success.ok_or_else(|| M::Error::missing_field("last_success"))?;

                let content = ScannedAssetMetadataAttributes {
                    asset,
                    first_success_timestamp,
                    last_success,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScannedAssetMetadataAttributesVisitor)
    }
}
