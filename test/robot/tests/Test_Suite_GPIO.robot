*** Settings ***
Documentation       GPIO

Resource            RaberryPico/platform.resource

Test Setup          Main Test Platform Setup
Test Teardown       Main Test Platform Cleanup

*** Test Cases ***

Check PIN are correctly set in OUTPUT/INPUT
    Given a system that is correctly initialized

    When the system is asked to set ${pins_list} as "OUTPUT"
    Then pins must be in "OUTPUT"

    When the system is asked to set ${pins_list} as "INPUT"
    Then pins must be in "INPUT"

Check than PIN in OUTPUT set correct value
    Given a system that is correctly initialized
    When the system is asked to set ${pins_list} as "OUTPUT"

    When value is set to "LOW"
    Then output value must be at "LOW"

    When value is set to "HIGH"
    Then output value must be at "HIGH"

Check than PIN in INPUT read correct value
    Given a system that is correctly initialized
    When the system is asked to set ${pins_list} as "INPUT"

    When input value is "LOW"
    Then value read must be "LOW"

    When input value is "HIGH"
    Then value read must be "HIGH"

Check FAILURE output when using wrong PIN
    Given a system that is correctly initialized
    When the system is asked to set 50 as "INPUT"

    Then output error must be FAILURE

Check FAILURE output when using wrong value in OUTPUT PIN
    Given a system that is correctly initialized
    When the system is asked to set 50 as "OUTPUT"

    Then output error must be FAILURE
    
*** Variables ***
${pins_list}=    [2,3,4,5,6,7,8,9]

*** Keywords ***
