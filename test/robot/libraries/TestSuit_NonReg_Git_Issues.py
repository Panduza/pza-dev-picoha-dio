#!/usr/bin/env python3
"""Test Suite of no regretion"""

import __future__

__author__ = "Jason PUEL"
__date__ = "12 Jan 2025"

# ================== Imports ===================

import logging

# local imports

import api_dio_pb2 as dio
from launcher_Python_test import *
from API_PicoHostAdapterDio import PicoHostAdapterDio, setup_logging

# ========= Test Suite of no regretion =========
# Check Git Issue


@test_launcher
def impossible_to_reset_pins(test: PicoHostAdapterDio):
    """
    Impossible to reset Pins to INTPUT #2
    After Set pin to OUTPUT, user is not able to Reset pin to INPUT
    """
    logging.info("Impossible to reset Pins to INTPUT #2")
    test.ping_info()
    test.set_gpio_direction(gpio=2, direction=dio.PinValue.OUTPUT)
    test.get_gpio_direction(gpio=2)
    test.set_gpio_direction(gpio=2, direction=dio.PinValue.INPUT)
    if test.get_gpio_direction(gpio=2) != dio.PinValue.INPUT:
        raise TestFailException("GPIO not re-set to INPUT.")


@test_launcher
def impossible_to_use_pin(test: PicoHostAdapterDio):
    """
    Impossible to use pin 26, 27 #3
    For some reason, Pin 26, 27 and 28 are not usable
    """
    logging.info("Impossible to use pin 26, 27 #3")
    test.ping_info()
    test.set_gpio_direction(gpio=26, direction=dio.PinValue.OUTPUT)
    get_26 = test.get_gpio_direction(gpio=26)
    test.set_gpio_direction(gpio=27, direction=dio.PinValue.OUTPUT)
    get_27 = test.get_gpio_direction(gpio=27)
    test.set_gpio_direction(gpio=28, direction=dio.PinValue.OUTPUT)
    get_28 = test.get_gpio_direction(gpio=28)

    # Check if Pin direction can be Set
    if (
        get_26 != dio.PinValue.OUTPUT
        or get_27 != dio.PinValue.OUTPUT
        or get_28 != dio.PinValue.OUTPUT
    ):
        raise TestFailException("GPIO not set to expected value.")

    set_26 = test.set_gpio_value(gpio=26, value=dio.PinValue.LOW)
    set_27 = test.set_gpio_value(gpio=27, value=dio.PinValue.LOW)
    set_28 = test.set_gpio_value(gpio=28, value=dio.PinValue.LOW)

    # Check if Pin are usable
    if (
        set_26 != dio.AnswerType.SUCCESS
        or set_27 != dio.AnswerType.SUCCESS
        or set_28 != dio.AnswerType.SUCCESS
    ):
        raise TestFailException("GPIO not set to expected value.")


@test_launcher
@timeout_wrapper(30)
def no_failure_when_using_not_existing_pins(test: PicoHostAdapterDio):
    """
    No FAILURE when using not existing PINs #4
    Here there is no FAILURE when using pin out of range AND it stuck the system
    """
    logging.info("No FAILURE when using not existing PINs #4")
    test.ping_info()
    if (
        test.set_gpio_direction(gpio=50, direction=dio.PinValue.OUTPUT)
        != dio.AnswerType.FAILURE
    ):
        raise TestFailException("There is not the expacted Failure.")

    if test.get_gpio_direction(gpio=50) != dio.AnswerType.FAILURE:
        raise TestFailException("There is not the expacted Failure.")
    test.ping_info()


@test_launcher
def fail_to_read_gpio_value(test: PicoHostAdapterDio):
    """
    Failure to read GPIO value #8
    When the OUTPUT GPIO's value is set to HIGH, we shall be able to 'GET_PIN_VALUE' with the INPUT GPIO
    """
    logging.info("Failure to read GPIO value #8")
    test.ping_info()
    test.set_gpio_direction(gpio=4, direction=dio.PinValue.INPUT)
    test.get_gpio_direction(gpio=4)
    test.set_gpio_direction(gpio=5, direction=dio.PinValue.OUTPUT)
    test.get_gpio_direction(gpio=5)

    test.set_gpio_value(gpio=5, value=dio.PinValue.HIGH)
    if test.get_gpio_value(gpio=4) != dio.PinValue.HIGH:
        raise TestFailException("GPIO not set to expected value.")

    test.set_gpio_value(gpio=5, value=dio.PinValue.LOW)
    if test.get_gpio_value(gpio=4) != dio.PinValue.LOW:
        raise TestFailException("GPIO not set to expected value.")


# ============= Main Fonctions =================

if __name__ == "__main__":
    """Create and run small test senario to help in Git issues validation"""

    # Setup
    logger = setup_logging(loggingLevel=logging.INFO)

    test = PicoHostAdapterDio("COM5")

    impossible_to_reset_pins(test)
    impossible_to_use_pin(test)
    no_failure_when_using_not_existing_pins(test)
    fail_to_read_gpio_value(test)

    print_results()
