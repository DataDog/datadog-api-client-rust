// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GuidedTableQuery {
	MetricsLikeQueryFragmentMetrics(Box<crate::datadogV1_20270101::model::MetricsLikeQueryFragmentMetrics>),
	MetricsLikeQueryFragmentCloudCost(Box<crate::datadogV1_20270101::model::MetricsLikeQueryFragmentCloudCost>),
	EventsQueryFragmentDefinition(Box<crate::datadogV1_20270101::model::EventsQueryFragmentDefinition>),
	ReferentialQueryFragmentReferenceTable(Box<crate::datadogV1_20270101::model::ReferentialQueryFragmentReferenceTable>),
	ReferentialQueryFragmentManagedResource(Box<crate::datadogV1_20270101::model::ReferentialQueryFragmentManagedResource>),
	PublishedAnalysisQueryFragment(Box<crate::datadogV1_20270101::model::PublishedAnalysisQueryFragment>),
	ApmMetricsQueryFragment(Box<crate::datadogV1_20270101::model::ApmMetricsQueryFragment>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for GuidedTableQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::MetricsLikeQueryFragmentMetrics>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GuidedTableQuery::MetricsLikeQueryFragmentMetrics(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::MetricsLikeQueryFragmentCloudCost>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GuidedTableQuery::MetricsLikeQueryFragmentCloudCost(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EventsQueryFragmentDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GuidedTableQuery::EventsQueryFragmentDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ReferentialQueryFragmentReferenceTable>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GuidedTableQuery::ReferentialQueryFragmentReferenceTable(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ReferentialQueryFragmentManagedResource>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GuidedTableQuery::ReferentialQueryFragmentManagedResource(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::PublishedAnalysisQueryFragment>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GuidedTableQuery::PublishedAnalysisQueryFragment(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ApmMetricsQueryFragment>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GuidedTableQuery::ApmMetricsQueryFragment(_v));
            }
        }
	
        return Ok(GuidedTableQuery::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
