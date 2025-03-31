*** Settings ***

# All imports must be done here
Library      Collections

# All variables must be defined here
Variables    ../../platform/RaspberryPico/platform_data.py
Variables    ../../libraries/api_dio_pb2.py

# All Library must be imported here 
Library     ../../libraries/python_utils.py
Library     ../../platform/RaspberryPico/platform_data.py
Library     ../../libraries/API_PicoHostAdapterDio.py

*** Variables ***
${Status_Last_CMD} =    None
${test}

*** Keywords ***
Main Test Platform Setup
    [Documentation]    Prepare testing environment
    Log     "RaspberryPico Setup"
    IF    '${PORT_COM_DUT}' == '${EMPTY}'
        ${PORT_COM_DUT}=    Find Port By Vid Pid    vid=5824    pid=1505
        Set Global Variable     ${PORT_COM_DUT}    ${PORT_COM_DUT}
    END
    Setup Context    port=${PORT_COM_DUT}

Main Test Platform Cleanup
    [Documentation]    Clean testing environment
    Log     "RaspberryPico Cleanup"
    Close Context

# Keywords can be place in other files to organize the code
a system that is correctly initialized
    [Documentation]    Check the system is correctly initialized
    Set Log Level    TRACE
    #Should Be Equal As Strings    ${test.is_open}    True
    #${ping}=    Call Method    ${test}    ping_info
    ${ping}=    Ping Info
    Should Be Equal As Strings    ${ping}    ${SUCCESS}
    Log                 "system is initialized"

communication working
    [Documentation]    Send Ping command
    ${ping}=    Ping Info
    Should Be Equal As Strings    ${ping}    ${SUCCESS}

the system is asked to set @{GPIOs_list} as "${direction}"
    [Documentation]    Set GPIOs and its pair to direction     
    # In platform setup we paired GPIO so we need to set GPIO direction 
    IF    ${direction} == ${INPUT}
        ${dir_com}=     Set Variable    ${OUTPUT}
    ELSE IF    ${direction} == ${OUTPUT}
        ${dir_com}=     Set Variable    ${INPUT}
    END

    FOR    ${gpio}   IN  @{GPIOs_list}
        # Set GPIO direction 
        ${gpio}=    Evaluate    ${gpio}
        ${status}=    Set Gpio Direction    gpio=${gpio}         direction=${direction}
        Should Be Equal As Strings    ${status}    ${SUCCESS}
        ${gpio_comp} =    Get Comp Gpio    ${gpio}
        ${status}=    Set Gpio Direction    gpio=${gpio_comp}    direction=${dir_com}
        Should Be Equal As Strings    ${status}    ${SUCCESS}
    END

@{GPIOs_list} must be in "${direction}"
    [Documentation]    Check gpio direction
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${gpio}=    Evaluate    ${gpio}
        ${gpio_dir}=    Get Gpio Direction    ${gpio}
        Should Be Equal As Strings    ${gpio_dir.value}    ${direction}
    END

@{GPIOs_list} value is set to "${value}"
    [Documentation]    "Set gpio value"
    FOR    ${gpio}   IN    @{GPIOs_list}
        ${gpio}=    Evaluate    ${gpio}
        ${status}=    Set Gpio Value    ${gpio}    ${value}
        Should Be Equal As Strings    ${status}    ${SUCCESS}
    END

@{GPIOs_list} output value must be at "${value}"
    [Documentation]    Check GPIO value output by reading paired GPIO input
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${gpio_comp} =    Get Comp Gpio    ${gpio}
        ${value_comp}=    Get Gpio Value     ${gpio_comp}
        Should Be Equal As Strings    ${value_comp.value}    ${value}
    END

@{GPIOs_list} input value is set to "${value}"
    [Documentation]    Set paired GPIO value
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${gpio_comp} =    Get Comp Gpio    ${gpio}
        ${status}=    Set Gpio Value    ${gpio_comp}    ${value}
        Should Be Equal As Strings    ${status}    ${SUCCESS}
    END

@{GPIOs_list} value read must be "${value}"
    [Documentation]    Check GPIO value input by reading paired GPIO output
    FOR    ${gpio}   IN  @{GPIOs_list}
        ${gpio_comp} =    Get Comp Gpio    ${gpio}
        ${value_read}=    Get Gpio Value    ${gpio_comp}
        Should Be Equal As Strings    ${value_read.value}    ${value}
    END

the system is asked to set a not existing gpio
    [Documentation]    Set GPIO 50 to OUTOUT
    ${not_existing_gpio}    Evaluate    50
    ${status}=    Run Keyword And Continue On Failure    
    ...    Set Gpio Direction    ${not_existing_gpio}     ${OUTPUT}
    Set Global Variable     ${Status_Last_CMD}    ${status}

output error must be FAILURE
    [Documentation]    Check the Status of the last command send
    Should Be Equal As Strings    ${Status_Last_CMD}    ${FAILURE}


check ping function
    [Documentation]    measure the execution time of a function
    [Arguments]    ${max_average_time}    ${repeat_time}=10
    # Get delta time 
    @{list_time_list} =    Create List  
    FOR    ${i}    IN RANGE    0    ${repeat_time}
        ${start_time}=    Evaluate    time.time()    modules=time
        Ping Info
        ${end_time}=    Evaluate    time.time()    modules=time
        ${elapsed_time}=    Evaluate    ${end_time} - ${start_time}
        Append To List    ${list_time_list}    ${elapsed_time}
    END
        # Calculate average
    ${sum}=    Evaluate    sum(${list_time_list})    modules=collections
    ${count}=    Get Length    ${list_time_list}
    ${average}=    Evaluate    ${sum} / ${count}

    # Add fail condition
    Run Keyword If    ${average} > ${max_average_time}   
    ...               Fail    Execution time exceeded ${max_average_time} second.

Check COM USB Info
    [Arguments]    ${product}    ${manufacturer}    ${vid}    ${pid}
    &{data_expected}    Create Dictionary    
    ...    product=${product} 
    ...    manufacturer=${manufacturer}
    ...    vid=${vid}
    ...    pid=${pid}
    Check USB Info    port_com_dut=${PORT_COM_DUT}  data_expected=&{data_expected}
