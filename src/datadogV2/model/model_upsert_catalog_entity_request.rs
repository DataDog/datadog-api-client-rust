// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Create / Update entity request.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum UpsertCatalogEntityRequest {
    EntityV3(Box<crate::datadogV2::model::EntityV3>),
    EntityRaw(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for UpsertCatalogEntityRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::EntityV3>>(value.clone())
        {
            match *_v {
                crate::datadogV2::model::EntityV3::UnparsedObject(_v) => {}
                _ => return Ok(UpsertCatalogEntityRequest::EntityV3(_v)),
            }
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(UpsertCatalogEntityRequest::EntityRaw(_v));
        }

        return Ok(UpsertCatalogEntityRequest::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
