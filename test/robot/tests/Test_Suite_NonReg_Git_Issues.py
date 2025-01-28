#!/usr/bin/env python3
"""
Test Suite of no regression
Describe small test scenarios to help validation of Git issues 
"""

import __future__

__author__ = "Jason PUEL"
__date__ = "12 Jan 2025"

# ================== Imports ===================

import logging, os, sys
import pytest

# local imports
# TODO : Remove this ASAP
sys.path.append(os.path.join(os.path.dirname(os.path.dirname(__file__)), "libraries"))

import api_dio_pb2 as dio
from launcher_Python_test import setup_test, timeout_wrapper
from API_PicoHostAdapterDio import PicoHostAdapterDio

# ========= Test Suite of no regression =========
# Check Git Issue


@setup_test
@timeout_wrapper()
def test_impossible_to_reset_pins(test: PicoHostAdapterDio):
    """
    Impossible to reset Pins to INTPUT #2
    After Set pin to OUTPUT, user is not able to Reset pin to INPUT
    """
    logging.info("Impossible to reset Pins to INTPUT #2")
    test.ping_info()

    test.set_gpio_direction(gpio=2, direction=dio.PinValue.OUTPUT)
    assert test.get_gpio_direction(gpio=2)

    test.set_gpio_direction(gpio=2, direction=dio.PinValue.INPUT)
    assert test.get_gpio_direction(gpio=2).value == dio.PinValue.INPUT


@setup_test
@timeout_wrapper()
def test_impossible_to_use_pin(test: PicoHostAdapterDio):
    """
    Impossible to use pin 26, 27 #3
    For some reason, Pin 26, 27 and 28 are not usable
    """
    logging.info("Impossible to use pin 26, 27 #3")
    test.ping_info()

    test.set_gpio_direction(gpio=26, direction=dio.PinValue.OUTPUT)
    assert test.get_gpio_direction(gpio=26).value == dio.PinValue.OUTPUT, "TOTO"
    test.set_gpio_direction(gpio=27, direction=dio.PinValue.OUTPUT)
    assert test.get_gpio_direction(gpio=27).value == dio.PinValue.OUTPUT
    test.set_gpio_direction(gpio=28, direction=dio.PinValue.OUTPUT)
    assert test.get_gpio_direction(gpio=28).value == dio.PinValue.OUTPUT

    assert (
        test.set_gpio_value(gpio=26, value=dio.PinValue.LOW) == dio.AnswerType.SUCCESS
    )
    assert (
        test.set_gpio_value(gpio=27, value=dio.PinValue.LOW) == dio.AnswerType.SUCCESS
    )
    assert (
        test.set_gpio_value(gpio=28, value=dio.PinValue.LOW) == dio.AnswerType.SUCCESS
    )


@setup_test
@timeout_wrapper()
def test_no_failure_when_using_not_existing_pins(test: PicoHostAdapterDio):
    """
    No FAILURE when using not existing PINs #4
    Here there is no FAILURE when using pin out of range AND it stuck the system
    """
    logging.info("No FAILURE when using not existing PINs #4")

    test.ping_info()

    assert (
        test.set_gpio_direction(gpio=50, direction=dio.PinValue.OUTPUT)
        == dio.AnswerType.FAILURE
    )

    assert test.get_gpio_direction(gpio=50).type == dio.AnswerType.FAILURE

    assert (
        test.set_gpio_value(gpio=50, value=dio.PinValue.HIGH) == dio.AnswerType.FAILURE
    )

    test.ping_info()


@setup_test
@timeout_wrapper()
def test_fail_to_read_gpio_value(test: PicoHostAdapterDio):
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
    assert test.get_gpio_value(gpio=4).value == dio.PinValue.HIGH

    test.set_gpio_value(gpio=5, value=dio.PinValue.LOW)
    assert test.get_gpio_value(gpio=4).value == dio.PinValue.LOW


# ============= Main Fonctions =================

if __name__ == "__main__":
    """Run small test scenarios to help in Git issues validation"""
    pytest.main(
        args=[
            "--capture=no",
            "--verbose",
            "--log-level=DEBUG",
            os.path.abspath(__file__),
        ]
    )
