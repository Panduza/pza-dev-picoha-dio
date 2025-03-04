Feature: Robustness Feature

  Scenario: Try to send corrupted data
    Given a serial connection to the device opened
    Given I send a corrupted data to the device
    When I send a ping command to the device
    Then I must receive a SUCCESS response from the device
    # Pin 99 doesn't exists
    When I send a set_direction "input" in pin "<inv_pin0>" command to the device
    Then I must receive a FAILURE response from the device
    When I send a ping command to the device
    Then I must receive a SUCCESS response from the device

    Examples:
      | inv_pin0 | pin_out |
      |       99 |       2 |
