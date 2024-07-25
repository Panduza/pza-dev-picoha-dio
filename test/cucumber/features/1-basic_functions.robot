*** Settings ***
Documentation       Basic functional tests for the PICoHA-DIO device

Resource            platform.resource

Test Setup          Main Test Platform Setup
Test Teardown       Main Test Platform Cleanup

*** Test Cases ***

Test Ping Function
    Given a serial connection to the device
    When I send a ping command to the device
    Then I should receive a SUCCESS response from the device

