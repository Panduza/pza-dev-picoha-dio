*** Settings ***
Documentation       GPIO

Resource            platform.resource

Test Setup          Main Test Platform Setup
Test Teardown       Main Test Platform Cleanup

*** Test Cases ***

Check Speed USB communication
    Given a system that is correctly initialized
    Then check ping function    max_average_time=2.2


*** Keywords ***
check ping function
    [Documentation]    measure the execution time of a function
    [Arguments]    ${max_average_time}    ${repeat_time}=10
    # Get delta time 
    @{list_time_list} =    Create List  
    FOR    ${i}    IN RANGE    0    ${repeat_time}
        ${start_time}=    Evaluate    time.time()    modules=time
        ping 
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


*** Variables ***

