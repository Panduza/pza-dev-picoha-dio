#!/usr/bin/env python3
"""This file link RobotFramework Keywords we python API"""
import logging

# local imports
import api_dio_pb2 as dio
from API_PicoHostAdapterDio import PicoHostAdapterDio


# ================== Keys World ================


def test_that_nothing_is_good_too():
    print("This is a test that does nothing, but it's good too.")
    print("You should be able to see this inside your test report.")


# ==============================================
test = None


def connect_to(COM: str = "COM4"):
    """Connect on Serial Port: COM"""
    global test
    try:
        if test:
            test.__del__()
        test = PicoHostAdapterDio(COM)
    except SyntaxError as err:
        raise SyntaxError(f"{err}")
    except:
        raise Exception(f"Imposible to create object PicoHost Adapter Dio on {COM}")


def disconnect():
    """Closed Serial Port connection"""
    try:
        test.__del__()
    except SyntaxError as err:
        raise SyntaxError(f"{err}")
    except:
        raise Exception(f"Imposible to delete object PicoHost Adapter Dio")


def is_connected():
    """Check connection on Serial Port"""
    try:
        return "true" if test.is_connected() else "false"
    except:
        raise Exception("Fail to connect product.")


def ping():
    """Send Ping frame"""
    ping_info = test.ping_info()
    if ping_info != dio.AnswerType.SUCCESS:
        raise ValueError("Not able to get a PING answer.")


def set_gpio_direction(gpio: int, direction: dio.PinValue):
    """Set GPIO direction"""
    if direction == "INPUT":
        err = test.set_gpio_direction(gpio, dio.PinValue.INPUT)
    elif direction == "OUTPUT":
        err = test.set_gpio_direction(gpio, dio.PinValue.OUTPUT)
    else:
        raise ValueError(f"This is not Direction value : {direction}")
    if err == dio.AnswerType.SUCCESS:
        return "SUCCESS"
    else:
        logging.warning(f"FAIL to set gpio direction")
        return "FAILURE"


def get_gpio_direction(gpio: int):
    """Return the GPIO direction"""
    direction = test.get_gpio_direction(gpio)
    if direction == dio.PinValue.INPUT:
        return "INPUT"
    elif direction == dio.PinValue.OUTPUT:
        return "OUTPUT"
    else:
        raise ValueError(f"FAIL to get GPIO{gpio} direction.")


def set_gpio_value(gpio: int, value):
    """Set GPIO direction"""
    if test.set_gpio_value(gpio, value) == dio.AnswerType.SUCCESS:
        return "SUCCESS"
    else:
        logging.warning(f"FAIL to set gpio value")
        return "FAILURE"


def get_gpio_value(gpio: int):
    """Return GPIO direction"""
    value = test.get_gpio_value(gpio)
    if value == dio.PinValue.LOW:
        return "LOW"
    elif value == dio.PinValue.HIGH:
        return "HIGH"
    else:
        raise ValueError(f"FAIL to get GPIO{gpio} value")


def check_gpio_direction(gpio: int, direction: str):
    """Verified for a given GPIO than its direction is set."""
    if not get_gpio_direction(gpio) == direction:
        raise ValueError(f"GPIO '{gpio}' is not set as {direction}")
    else:
        return True


def check_gpio_value(gpio: int, value):
    """Verified for a given GPIO than its value is set by reading the input on its paired GPIO"""
    if not get_gpio_value(gpio) == value:
        raise ValueError(f"GPIO '{gpio} is not set as {value}")
    else:
        return True


# ================== Main ======================
if __name__ == "__main__":
    help(__name__)
