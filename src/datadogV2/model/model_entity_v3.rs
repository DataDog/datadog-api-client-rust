// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Entity schema v3.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EntityV3 {
    EntityV3Service(Box<crate::datadogV2::model::EntityV3Service>),
    EntityV3Datastore(Box<crate::datadogV2::model::EntityV3Datastore>),
    EntityV3Queue(Box<crate::datadogV2::model::EntityV3Queue>),
    EntityV3System(Box<crate::datadogV2::model::EntityV3System>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for EntityV3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::EntityV3Service>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EntityV3::EntityV3Service(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::EntityV3Datastore>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EntityV3::EntityV3Datastore(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::EntityV3Queue>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EntityV3::EntityV3Queue(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::EntityV3System>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EntityV3::EntityV3System(_v));
            }
        }

        return Ok(EntityV3::UnparsedObject(crate::datadog::UnparsedObject {
            value,
        }));
    }
}
