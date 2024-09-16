*** Settings ***

# All imports must be done here
Library      Collections

# All variables must be defined here
Variables    platform_data.py

# All Library must be imported here 
Library      ../../libraries/gpio_keywords.py

*** Variables ***
# I will try with few pins
${Status_Last_CMD} =    SUCCESS

*** Keywords ***

Main Test Platform Setup
    Set Log Level    DEBUG
    Log     "RaspberryPico Setup"
    a system that is correctly initialized

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
   
the system is asked to set @{pins_list} as "${mode}"
    FOR    ${pin}   IN    @{pins_list}
        ${Status_Last_CMD}    set gpio mode    ${pin}     ${mode}   
    END

@{pins_list} must be in "${mode}"
    FOR    ${pin}   IN  @{pins_list}
        ${moderead}    get gpio mode    ${pin}
        Should Be Equal     ${mode}    ${moderead} 
    END

@{pins_list} value is set to "${value}"
    FOR    ${pin}   IN  @{pins_list}
        ${Status_Last_CMD}    set gpio value    ${pin}    ${value}
    END

@{pins_list} output value must be at "${value}"
    FOR    ${pin}   IN  @{pins_list}
        ${value_comp}   get gpio value comp     ${pin}
        Should Be Equal     ${value}    ${value_comp}
    END

@{pins_list} input value is "${value}"
    FOR    ${pin}   IN  @{pins_list}
        Log "${pin} tested: comp set to ${value}"
        ${Status_Last_CMD}    set gpio value comp    ${pin}    ${value}
    END

@{pins_list} value read must be "${value}"
    FOR    ${pin}   IN  @{pins_list}
        ${value_comp}   get gpio value  ${pin}
        Log "${pin} tested: comp set to ${value}"
        Should Be Equal     ${value}    ${value_comp}
    END

the system is asked to set a not existing pin
    ${Status_Last_CMD}    set gpio mode    50     ${mode} 

output error must be FAILURE
    Should Be Equal    ${Status_Last_CMD}   FAILURE

