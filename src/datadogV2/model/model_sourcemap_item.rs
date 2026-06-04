// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A source map data object representing one of the supported map kinds.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SourcemapItem {
    JSSourcemapData(Box<crate::datadogV2::model::JSSourcemapData>),
    ReactNativeSourcemapData(Box<crate::datadogV2::model::ReactNativeSourcemapData>),
    IOSSourcemapData(Box<crate::datadogV2::model::IOSSourcemapData>),
    JVMSourcemapData(Box<crate::datadogV2::model::JVMSourcemapData>),
    FlutterSourcemapData(Box<crate::datadogV2::model::FlutterSourcemapData>),
    ELFSourcemapData(Box<crate::datadogV2::model::ELFSourcemapData>),
    NDKSourcemapData(Box<crate::datadogV2::model::NDKSourcemapData>),
    IL2CPPSourcemapData(Box<crate::datadogV2::model::IL2CPPSourcemapData>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SourcemapItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::JSSourcemapData>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(SourcemapItem::JSSourcemapData(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ReactNativeSourcemapData>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SourcemapItem::ReactNativeSourcemapData(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::IOSSourcemapData>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(SourcemapItem::IOSSourcemapData(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::JVMSourcemapData>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(SourcemapItem::JVMSourcemapData(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::FlutterSourcemapData>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SourcemapItem::FlutterSourcemapData(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ELFSourcemapData>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(SourcemapItem::ELFSourcemapData(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::NDKSourcemapData>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(SourcemapItem::NDKSourcemapData(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::IL2CPPSourcemapData>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SourcemapItem::IL2CPPSourcemapData(_v));
            }
        }

        return Ok(SourcemapItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
