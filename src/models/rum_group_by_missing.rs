/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RumGroupByMissing : The value to use for logs that don't have the facet used to group by.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RumGroupByMissing {
}

impl RumGroupByMissing {
    /// The value to use for logs that don't have the facet used to group by.
    pub fn new() -> RumGroupByMissing {
        RumGroupByMissing {
        }
    }
}


