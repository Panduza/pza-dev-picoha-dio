Feature: Robustness Feature

  Scenario: Try to send corrupted data
    Given a serial connection to the device opened
    Given I send a corrupted data to the device
    When I send a ping command to the device
    Then I must receive a SUCCESS response from the device
