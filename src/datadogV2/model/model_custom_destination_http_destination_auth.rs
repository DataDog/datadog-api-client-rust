// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Authentication method of the HTTP requests.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CustomDestinationHttpDestinationAuth {
    CustomDestinationHttpDestinationAuthBasic(
        Box<crate::datadogV2::model::CustomDestinationHttpDestinationAuthBasic>,
    ),
    CustomDestinationHttpDestinationAuthCustomHeader(
        Box<crate::datadogV2::model::CustomDestinationHttpDestinationAuthCustomHeader>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CustomDestinationHttpDestinationAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CustomDestinationHttpDestinationAuthBasic>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    CustomDestinationHttpDestinationAuth::CustomDestinationHttpDestinationAuthBasic(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CustomDestinationHttpDestinationAuthCustomHeader>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CustomDestinationHttpDestinationAuth::CustomDestinationHttpDestinationAuthCustomHeader(_v));
            }
        }

        return Ok(CustomDestinationHttpDestinationAuth::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
