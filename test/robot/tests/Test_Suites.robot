*** Settings ***
Documentation       This file Describe tests and run it.

Resource            ../platform/RaspberryPico/platform.resource

Test Setup          Main Test Platform Setup
Test Teardown       Main Test Platform Cleanup

*** Test Cases ***

Check GPIO are correctly set in OUTPUT/INPUT
    [Tags]    GPIO
    Given a system that is correctly initialized

    When the system is asked to set @{GPIOs_list} as "${OUTPUT}"
    Then @{GPIOs_list} must be in "${OUTPUT}"

    When the system is asked to set @{GPIOs_list} as "${INPUT}"
    Then @{GPIOs_list} must be in "${INPUT}"

Check than GPIO in OUTPUT set correct value
    [Tags]    GPIO
    Given a system that is correctly initialized
    When the system is asked to set @{GPIOs_list} as "${OUTPUT}"

    When @{GPIOs_list} value is set to "${LOW}"
    Then @{GPIOs_list} output value must be at "${LOW}"

    When @{GPIOs_list} value is set to "${HIGH}"
    Then @{GPIOs_list} output value must be at "${HIGH}"

Check than GPIO in INPUT read correct value
    [Tags]    GPIO
    Given a system that is correctly initialized
    When the system is asked to set @{GPIOs_list} as "${INPUT}"

    When @{GPIOs_list} input value is set to "${LOW}"
    Then @{GPIOs_list} value read must be "${LOW}"

    When @{GPIOs_list} input value is set to "${HIGH}"
    Then @{GPIOs_list} value read must be "${HIGH}"

Check system still working after FAILURE
    [Tags]    GPIO
    Given a system that is correctly initialized
    When the system is asked to set a not existing gpio

    Then communication working

Check FAILURE output when using wrong GPIO
    [Tags]    GPIO
    Given a system that is correctly initialized
    When the system is asked to set a not existing gpio

    Then output error must be FAILURE

Check Speed USB communication
    [Tags]    USB
    Given a system that is correctly initialized
    Then check ping function    max_average_time=2.2

Check Information present in USB COM Info
    [Tags]    USB
    Given a system that is correctly initialized
    Then Check COM USB Info    
    ...    product=picoha-dio    manufacturer=panduza
    ...    vid=5824    pid=1505
    

*** Variables ***
@{GPIOs_list}    2    4    6    8    10    12    14    16    18    20    22    27
