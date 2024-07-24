*** Settings ***
Documentation       Example of Gherkin syntax

Resource            platform.resource

Test Setup          Main Test Platform Setup
Test Teardown       Main Test Platform Cleanup

*** Test Cases ***

Test corrupted data
    Given a serial connection to the device
    When I send "100" random bytes to the device
    Then the device must still be able to respond correctly

*** Keywords ***

I send "${number_of_bytes}" random bytes to the device
    Log    message: ${number_of_bytes}
    Log    need to implement
