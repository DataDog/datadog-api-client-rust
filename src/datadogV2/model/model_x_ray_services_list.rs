// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// AWS X-Ray services to collect traces from. Defaults to `include_only`.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum XRayServicesList {
    XRayServicesIncludeAll(Box<crate::datadogV2::model::XRayServicesIncludeAll>),
    XRayServicesIncludeOnly(Box<crate::datadogV2::model::XRayServicesIncludeOnly>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for XRayServicesList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::XRayServicesIncludeAll>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(XRayServicesList::XRayServicesIncludeAll(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::XRayServicesIncludeOnly>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(XRayServicesList::XRayServicesIncludeOnly(_v));
            }
        }

        return Ok(XRayServicesList::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
