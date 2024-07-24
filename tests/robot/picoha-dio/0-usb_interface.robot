*** Settings ***
Documentation       First test usb interface

Resource            platform.resource

Test Setup          Main Test Platform Setup
Test Teardown       Main Test Platform Cleanup

*** Test Cases ***

Test Ping Function
    When I list all the USB devices
    Then I want to find my device with vid "0x0001" and pid "0x0001"

*** Keywords ***

I want to find my device with vid "${vid}" and pid "${pid}"
    Log    message: ${vid} ${pid}
    Log    need to implement
