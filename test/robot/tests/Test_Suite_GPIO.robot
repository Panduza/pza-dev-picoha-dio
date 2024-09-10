*** Settings ***
Documentation       GPIO

Resource            RaberryPico/platform.resource

Test Setup          Main Test Platform Setup
Test Teardown       Main Test Platform Cleanup

*** Test Cases ***

Check PIN are correctly set in OUTPUT/INPUT
    Given a system that is correctly initialized

    When the system is asked to set ${pins_list} as "OUTPUT"
    Then ${pins_list} must be in "OUTPUT"

    When the system is asked to set ${pins_list} as "INPUT"
    Then pins must be in "INPUT"

Check than PIN in OUTPUT set correct value
    Given a system that is correctly initialized
    When the system is asked to set ${pins_list} as "OUTPUT"

    When ${pins_list} value is set to "LOW"
    Then ${pins_list} output value must be at "LOW"

    When ${pins_list} value is set to "HIGH"
    Then ${pins_list} output value must be at "HIGH"

Check than PIN in INPUT read correct value
    Given a system that is correctly initialized
    When the system is asked to set ${pins_list} as "INPUT"

    When ${pins_list} input value is "LOW"
    Then ${pins_list} value read must be "LOW"

    When ${pins_list} input value is "HIGH"
    Then ${pins_list} value read must be "HIGH"

Check FAILURE output when using wrong PIN
    Given a system that is correctly initialized
    When the system is asked to set 50 as "INPUT"

    Then output error must be FAILURE

Check system still working after FAILURE
    Given a system that is correctly initialized
    When the system is asked to set 50 as "OUTPUT"
    When output error must be FAILURE

    Then communication working
    
*** Variables ***
${pins_list}=    [2,4,6,8,10,12,14,16,18,20,22,24,27]

