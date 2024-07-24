*** Settings ***
Documentation       Example of Gherkin syntax

Resource            platform.resource

Test Setup          Main Test Platform Setup
Test Teardown       Main Test Platform Cleanup

*** Test Cases ***

Test corrupted data
    Given A serial connection to the device is opened
    When I send "100" random bytes to the device
    Then The device must still be able to respond correctly

*** Keywords ***

# A serial connection to the device is opened
    

I send "${number_of_bytes}" random bytes to the device
    Log    message: ${number_of_bytes}
    Log    need to implement



