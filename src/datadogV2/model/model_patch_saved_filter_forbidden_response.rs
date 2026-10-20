// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An authentication or permission error, or a resource listing consumer flags that the caller cannot edit.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PatchSavedFilterForbiddenResponse {
    SavedFilterAPIErrorResponse(Box<crate::datadogV2::model::SavedFilterAPIErrorResponse>),
    SavedFilterJSONAPIErrorResponse(Box<crate::datadogV2::model::SavedFilterJSONAPIErrorResponse>),
    SavedFilterPermissionDeniedResponse(
        Box<crate::datadogV2::model::SavedFilterPermissionDeniedResponse>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for PatchSavedFilterForbiddenResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SavedFilterAPIErrorResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(PatchSavedFilterForbiddenResponse::SavedFilterAPIErrorResponse(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SavedFilterJSONAPIErrorResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(PatchSavedFilterForbiddenResponse::SavedFilterJSONAPIErrorResponse(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SavedFilterPermissionDeniedResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    PatchSavedFilterForbiddenResponse::SavedFilterPermissionDeniedResponse(_v),
                );
            }
        }

        return Ok(PatchSavedFilterForbiddenResponse::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
