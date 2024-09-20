*** Settings ***

# All imports must be done here
Library      Collections

# All variables must be defined here
Variables    platform_data.py

# All Library must be imported here 
Library      ../../libraries/gpio_keywords.py

*** Variables ***
${Status_Last_CMD} =    None

*** Keywords ***

Main Test Platform Setup
    Set Log Level    DEBUG
    Log     "RaspberryPico Setup"

Main Test Platform Cleanup
    Log     "RaspberryPico Cleanup"


# Keywords can be place in other files to organize the code
a system that is correctly initialized
    ${system_state}     is connected
    Should Be Equal     ${system_state}    true
    ${ping}    ping
    Should Be Equal     ${ping}    true
    Log                 "system is initialized"

communication working
    ${ping}    ping
    Should Be Equal     ${ping}    true
   
the system is asked to set @{GPIOs_list} as "${direction}"
    FOR    ${gpio}   IN    @{GPIOs_list}
        ${Status}    set gpio direction and his comp    ${gpio}     ${direction}   
    END

@{GPIOs_list} must be in "${direction}"
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${directionread}    get gpio direction    ${gpio}
        Should Be Equal     ${direction}    ${directionread} 
    END

@{GPIOs_list} value is set to "${value}"
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${Status}    set gpio value    ${gpio}    ${value}
    END

@{GPIOs_list} output value must be at "${value}"
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${value_comp}   get gpio value comp     ${gpio}
        Should Be Equal     ${value}    ${value_comp}
    END

@{GPIOs_list} input value is "${value}"
    FOR    ${gpio}   IN  @{GPIOs_list}
        Log    "${gpio} under test: comp set to ${value}"
        ${Status}    set gpio value comp    ${gpio}    ${value}
    END

@{GPIOs_list} value read must be "${value}"
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${value_comp}   get gpio value  ${gpio}
        Log    "${gpio} under test: comp set to ${value}"
        Should Be Equal     ${value}    ${value_comp}
    END

the system is asked to set a not existing gpio
    ${Status}    set gpio direction    50     OUTPUT 
    Set Global Variable     ${Status_Last_CMD}    ${Status} 

output error must be FAILURE
    Should Be Equal    ${Status_Last_CMD}   FAILURE

