// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A custom destination's location to forward logs.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CustomDestinationResponseForwardDestination {
    CustomDestinationResponseForwardDestinationHttp(
        Box<crate::datadogV2::model::CustomDestinationResponseForwardDestinationHttp>,
    ),
    CustomDestinationResponseForwardDestinationSplunk(
        Box<crate::datadogV2::model::CustomDestinationResponseForwardDestinationSplunk>,
    ),
    CustomDestinationResponseForwardDestinationElasticsearch(
        Box<crate::datadogV2::model::CustomDestinationResponseForwardDestinationElasticsearch>,
    ),
    CustomDestinationResponseForwardDestinationMicrosoftSentinel(
        Box<crate::datadogV2::model::CustomDestinationResponseForwardDestinationMicrosoftSentinel>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CustomDestinationResponseForwardDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CustomDestinationResponseForwardDestinationHttp>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CustomDestinationResponseForwardDestination::CustomDestinationResponseForwardDestinationHttp(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CustomDestinationResponseForwardDestinationSplunk>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CustomDestinationResponseForwardDestination::CustomDestinationResponseForwardDestinationSplunk(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CustomDestinationResponseForwardDestinationElasticsearch>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CustomDestinationResponseForwardDestination::CustomDestinationResponseForwardDestinationElasticsearch(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::CustomDestinationResponseForwardDestinationMicrosoftSentinel>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CustomDestinationResponseForwardDestination::CustomDestinationResponseForwardDestinationMicrosoftSentinel(_v));
            }
        }

        return Ok(CustomDestinationResponseForwardDestination::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
