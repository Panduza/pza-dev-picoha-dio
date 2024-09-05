*** Settings ***

# All imports must be done here
Library      Collections

# All variables must be defined here
Variables    platform_data.py

# All Library must be imported here 
Library      gpio_keywords.py

*** Variables ***
# I will try with few pins
${pin}=    2

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
    Log                 "system is initialized"
   
the system is asked to set ${pins_list} as "${mode}"
    Log    "try set 2 in ${mode}"
    set pin mode    ${pin}     ${mode}

pins must be in "${mode}"
    ${mode}    get pin mode    ${pin}
    Log    "check 2 are in ${mode}"

value is set to "${value}"
    Log    "value is set to"

output value must be at "${value}"
    Log    "output value must be at"

input value is "${value}"
    Log    "input value is"

value read must be "${value}"
    Log    "value read must be"
