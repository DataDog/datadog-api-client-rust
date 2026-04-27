@endpoint(rum-permanent-retention-filters) @endpoint(rum-permanent-retention-filters-v2)
Feature: Rum Permanent Retention Filters
  Manage permanent retention filters through [Manage
  Applications](https://app.datadoghq.com/rum/list) in RUM for your
  organization.  Permanent retention filters are preconfigured filters that
  ship with each RUM application. They cannot be created, deleted, or
  reordered. Their identity (`name`, `event_type`, `query`, `sample_rate`,
  `enabled`) and cross-product enabled flags (`session_replay_enabled`,
  `trace_enabled`) are fixed. You can adjust only the cross-product sample
  rates (`session_replay_sample_rate`, `trace_sample_rate`), and only when
  the corresponding editability flag is `true`.

  Background:
    Given a valid "apiKeyAuth" key in the system
    And a valid "appKeyAuth" key in the system
    And an instance of "RumPermanentRetentionFilters" API

  @team:DataDog/rum-backend
  Scenario: Get a permanent retention filter returns "Not Found" response
    Given new "GetPermanentRetentionFilter" request
    And request contains "app_id" parameter with value "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690"
    And request contains "rf_id" parameter with value "{{ unique }}"
    When the request is sent
    Then the response status is 404 Not Found

  @replay-only @team:DataDog/rum-backend
  Scenario: Get a permanent retention filter returns "OK" response
    Given new "GetPermanentRetentionFilter" request
    And request contains "app_id" parameter with value "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690"
    And request contains "rf_id" parameter with value "default_synthetics"
    When the request is sent
    Then the response status is 200 OK
    And the response "data.id" is equal to "default_synthetics"
    And the response "data.type" is equal to "permanent_retention_filters"

  @replay-only @team:DataDog/rum-backend
  Scenario: Get all permanent retention filters returns "OK" response
    Given new "ListPermanentRetentionFilters" request
    And request contains "app_id" parameter with value "1d4b9c34-7ac4-423a-91cf-9902d926e9b3"
    When the request is sent
    Then the response status is 200 OK

  @team:DataDog/rum-backend
  Scenario: Update a permanent retention filter returns "Bad Request" response
    Given new "UpdatePermanentRetentionFilter" request
    And request contains "app_id" parameter with value "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690"
    And request contains "rf_id" parameter with value "{{ unique }}"
    And body with value {"data":{"id":"{{ unique }}","type":"invalid_type","attributes":{"cross_product_sampling":{"session_replay_sample_rate":75,"trace_sample_rate":25}}}}
    When the request is sent
    Then the response status is 400 Bad Request

  @team:DataDog/rum-backend
  Scenario: Update a permanent retention filter returns "Not Found" response
    Given new "UpdatePermanentRetentionFilter" request
    And request contains "app_id" parameter with value "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690"
    And request contains "rf_id" parameter with value "{{ unique }}"
    And body with value {"data":{"id":"{{ unique }}","type":"permanent_retention_filters","attributes":{"cross_product_sampling":{"trace_sample_rate":25}}}}
    When the request is sent
    Then the response status is 404 Not Found

  @replay-only @team:DataDog/rum-backend
  Scenario: Update a permanent retention filter returns "Updated" response
    Given new "UpdatePermanentRetentionFilter" request
    And request contains "app_id" parameter with value "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690"
    And request contains "rf_id" parameter with value "default_replays"
    And body with value {"data":{"id":"default_replays","type":"permanent_retention_filters","attributes":{"cross_product_sampling":{"trace_sample_rate":100}}}}
    When the request is sent
    Then the response status is 200 Updated
    And the response "data.id" is equal to "default_replays"
    And the response "data.type" is equal to "permanent_retention_filters"
