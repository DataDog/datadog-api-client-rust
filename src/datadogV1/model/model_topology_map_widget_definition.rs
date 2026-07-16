// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// This widget displays a topology of nodes and edges for different data sources. It replaces the service map widget.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TopologyMapWidgetDefinition {
    TopologyMapWidgetDefinitionDataStreams(
        Box<crate::datadogV1::model::TopologyMapWidgetDefinitionDataStreams>,
    ),
    TopologyMapWidgetDefinitionServiceMap(
        Box<crate::datadogV1::model::TopologyMapWidgetDefinitionServiceMap>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TopologyMapWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TopologyMapWidgetDefinitionDataStreams>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TopologyMapWidgetDefinition::TopologyMapWidgetDefinitionDataStreams(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TopologyMapWidgetDefinitionServiceMap>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TopologyMapWidgetDefinition::TopologyMapWidgetDefinitionServiceMap(_v));
            }
        }

        return Ok(TopologyMapWidgetDefinition::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
