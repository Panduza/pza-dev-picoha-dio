*** Settings ***

# All imports must be done here
Library      Collections

# All variables must be defined here
Variables    platform_data.py

# All Library must be imported here 
Library      ../../libraries/gpio_keywords.py
Library     platform_data.py

*** Variables ***
${Status_Last_CMD} =    None
${COM} =     COM4

*** Keywords ***

Main Test Platform Setup
    [Documentation]    Prepare testing environement
    Set Log Level    DEBUG
    Connect to    ${COM} 
    Log     "RaspberryPico Setup"

Main Test Platform Cleanup
    [Documentation]    Clean testing environement
    Disconnect
    Log     "RaspberryPico Cleanup"


# Keywords can be place in other files to organize the code
a system that is correctly initialized
    [Documentation]    Check the systeme is correctly initialized
    is connected
    ping
    Log                 "system is initialized"

communication working
    [Documentation]    Send Ping commend
    ${ping}    ping
   
the system is asked to set @{GPIOs_list} as "${direction}"
    [Documentation]    Set GPIOs and its paire to direction 
    ...                - GPIOs_list : list of int 
    ...                - direction : string [ INPUT / OUTPUT ]
    FOR    ${gpio}   IN    @{GPIOs_list}
        ${gpio_comp} =    Get Comp Gpio    ${gpio}
        IF  ("${direction}" == "INPUT")
            set gpio direction    ${gpio}     INPUT
            set gpio direction    ${gpio_comp}     OUTPUT
        ELSE IF  ("${direction}" == "OUTPUT")
            set gpio direction    ${gpio}     OUTPUT
            set gpio direction    ${gpio_comp}     INPUT
        ELSE
            Log    ${direction} is not a valid direction    level=WARN
        END
    END

@{GPIOs_list} must be in "${direction}"
    [Documentation]    Check gpio direction
    FOR    ${gpio}   IN  @{GPIOs_list}
        Run Keyword And Continue On Failure    check gpio direction     ${gpio}    ${direction}  
    END

@{GPIOs_list} value is set to "${value}"
    [Documentation]    "Set gpio value"
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${Status}    set gpio value    ${gpio}    ${value}
    END

@{GPIOs_list} output value must be at "${value}"
    [Documentation]    Check GPIO value output by reading paired GPIO input
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${gpio_comp} =    Get Comp Gpio    ${gpio}
        Run Keyword And Continue On Failure    check gpio value     ${gpio_comp}    ${value}
    END

@{GPIOs_list} input value is set to "${value}"
    [Documentation]    Set paired GPIO value
    FOR    ${gpio}   IN  @{GPIOs_list}
        Log    "${gpio} under test: comp set to ${value}"
        ${gpio_comp} =    Get Comp Gpio    ${gpio}
        ${Status}    set gpio value    ${gpio}    ${value}
    END

@{GPIOs_list} value read must be "${value}"
    [Documentation]    Check GPIO value input by reading paired GPIO output
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${gpio_comp} =    Get Comp Gpio    ${gpio}
        Run Keyword And Continue On Failure    Check Gpio Value    ${gpio_comp}     ${value}
    END

the system is asked to set a not existing gpio
    [Documentation]    Set GPIO 50 to OUTOUT
    ${Status}    set gpio direction    50     OUTPUT 
    Set Global Variable     ${Status_Last_CMD}    ${Status} 

output error must be FAILURE
    [Documentation]    Check the Status of the last commend send
    IF    ${Status_Last_CMD} == FAILURE
        Log    Output error is not correct    level=ERROR
    END

