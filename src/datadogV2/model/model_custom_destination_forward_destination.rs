// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A custom destination's location to forward logs.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CustomDestinationForwardDestination {
    CustomDestinationForwardDestinationHttp(
        Box<crate::datadogV2::model::CustomDestinationForwardDestinationHttp>,
    ),
    CustomDestinationForwardDestinationSplunk(
        Box<crate::datadogV2::model::CustomDestinationForwardDestinationSplunk>,
    ),
    CustomDestinationForwardDestinationElasticsearch(
        Box<crate::datadogV2::model::CustomDestinationForwardDestinationElasticsearch>,
    ),
    CustomDestinationForwardDestinationMicrosoftSentinel(
        Box<crate::datadogV2::model::CustomDestinationForwardDestinationMicrosoftSentinel>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CustomDestinationForwardDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CustomDestinationForwardDestinationHttp>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    CustomDestinationForwardDestination::CustomDestinationForwardDestinationHttp(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CustomDestinationForwardDestinationSplunk>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    CustomDestinationForwardDestination::CustomDestinationForwardDestinationSplunk(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CustomDestinationForwardDestinationElasticsearch>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CustomDestinationForwardDestination::CustomDestinationForwardDestinationElasticsearch(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CustomDestinationForwardDestinationMicrosoftSentinel>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CustomDestinationForwardDestination::CustomDestinationForwardDestinationMicrosoftSentinel(_v));
            }
        }

        return Ok(CustomDestinationForwardDestination::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
