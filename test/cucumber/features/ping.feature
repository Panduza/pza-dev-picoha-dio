Feature: Animal feature

  Scenario: If we ping the device answer
    Given a serial connection to the device opened
    When I send a ping command to the device
    Then I must receive a SUCCESS response from the device
