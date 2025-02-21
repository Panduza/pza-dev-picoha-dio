Feature: Benchmark Feature

  Scenario: Do benchmark
    Given a serial connection to the device opened
    When Do benchmark
    Then I must receive a SUCCESS response from the device
