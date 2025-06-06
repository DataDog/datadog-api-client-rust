// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Create or update kind request.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum UpsertCatalogKindRequest {
    KindObj(Box<crate::datadogV2::model::KindObj>),
    KindRaw(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for UpsertCatalogKindRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::KindObj>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(UpsertCatalogKindRequest::KindObj(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(UpsertCatalogKindRequest::KindRaw(_v));
        }

        return Ok(UpsertCatalogKindRequest::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
