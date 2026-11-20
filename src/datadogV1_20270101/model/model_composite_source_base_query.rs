// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CompositeSourceBaseQuery {
	FormulaMetricsQueryMetrics(Box<crate::datadogV1_20270101::model::FormulaMetricsQueryMetrics>),
	FormulaMetricsQueryCloudCost(Box<crate::datadogV1_20270101::model::FormulaMetricsQueryCloudCost>),
	FormulaMetricsQueryDatadogUsage(Box<crate::datadogV1_20270101::model::FormulaMetricsQueryDatadogUsage>),
	CompositeEventsPlatformQueryDefinition(Box<crate::datadogV1_20270101::model::CompositeEventsPlatformQueryDefinition>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CompositeSourceBaseQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaMetricsQueryMetrics>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CompositeSourceBaseQuery::FormulaMetricsQueryMetrics(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaMetricsQueryCloudCost>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CompositeSourceBaseQuery::FormulaMetricsQueryCloudCost(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaMetricsQueryDatadogUsage>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CompositeSourceBaseQuery::FormulaMetricsQueryDatadogUsage(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CompositeEventsPlatformQueryDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CompositeSourceBaseQuery::CompositeEventsPlatformQueryDefinition(_v));
            }
        }
	
        return Ok(CompositeSourceBaseQuery::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
