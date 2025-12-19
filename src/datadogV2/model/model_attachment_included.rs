// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AttachmentIncluded {
    User140420082644000(Box<crate::datadogV2::model::User140420082644000>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AttachmentIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::User140420082644000>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(AttachmentIncluded::User140420082644000(_v));
            }
        }

        return Ok(AttachmentIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
