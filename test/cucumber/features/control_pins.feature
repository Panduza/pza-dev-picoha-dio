Feature: Control Pin Feature
  # ------
  # Scenario: If we set the led pin to output then high, led should be on
  #   Given a serial connection to the device opened
  #   When I send a set_direction "output" in pin "25" command to the device
  #   When I send a set_value "high" in pin "25" command to the device
  #   Then I must receive a SUCCESS response from the device
  # ------
  # Scenario Outline: Check that all pins can be turned on then off
  #   Given a serial connection to the device opened
  #   When I send a set_direction "output" in pin "<pin>" command to the device
  #   When I send a set_value "high" in pin "<pin>" command to the device
  #   Then I must receive a SUCCESS response from the device
  #   When I send a set_value "low" in pin "<pin>" command to the device
  #   Then I must receive a SUCCESS response from the device
  #   Examples:
  #     | pin |
  #     |   3 |
  #     |   4 |
  #     |  25 |
  # ------

  Scenario Outline: Loopback Scenario
    Given a serial connection to the device opened
    When I send a set_direction "output" in pin "<pin_out>" command to the device
    When I send a set_direction "output" in pin "<pin_in>" command to the device
    When I send a set_value "low" in pin "<pin_in>" command to the device
    When I send a set_value "high" in pin "<pin_out>" command to the device
    Then I must receive a SUCCESS response from the device
    # When I wait for 2 seconds
    # When I send a set_value "low" in pin "<pin_out>" command to the device
    # Then I must receive a SUCCESS response from the device

    Examples:
      | pin_out | pin_in |
      |       2 |      3 |
      # |       3 |      2 |
