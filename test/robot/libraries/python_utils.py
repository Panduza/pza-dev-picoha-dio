#!/usr/bin/env python3
"""
Utilities :
RobotFramework Keywords
Decorator to manage python test and useful function are describe here.
"""

import __future__

__author__ = "Jason PUEL"
__date__ = "12 Jan 2025"

# ================== Imports ===================

import threading
import functools
import logging
from serial.tools import list_ports

# Local imports
from platform_data import PORT_COM_DUT
from API_PicoHostAdapterDio import PicoHostAdapterDio


# ================== Utilities =================
###
# This part add RobotFramework Keywords to compete python API
###
def check_usb_info(port_com_dut: str, data_expected: dict):
    """
    Verified every expected serial COM info are present

    port_com_dut: str
    data_expected: dict
    """
    for port in list_ports.comports():
        if port_com_dut == port.device:
            logging.info(f"Serial Port found: {port.__dict__}")
            if all(field in port.__dict__ for field in data_expected):
                raise ValueError("Every required data not present in port COM info.")
            break


def find_port_by_vid_pid(vid: int, pid: int) -> str:
    """
    Return the first serial port found on the system with the specified VID and PID.

    :param vid: The Vendor ID of the device.
    :param pid: The Product ID of the device.
    :returns: The first serial port that matches the VID and PID.
    """
    result = None
    for port in list_ports.comports():
        if port.vid == vid and port.pid == pid:
            logging.debug(f"VID:PID matching on {port.serial_number} : {port.device}")
            result = port.device
            break
    return result


# ================== Class =====================


class TimeoutException(Exception):
    # Do nothing only handle timeout error
    pass


# ================== Decorator =================


def timeout_wrapper(timeout=60):
    """
    Launch a function and break it on timeout.
    Default timeout: 60 sec.
    """

    def decorator(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            result = [None]

            def target():
                try:
                    result[0] = func(*args, **kwargs)
                except Exception as e:
                    result[0] = e

            thread = threading.Thread(target=target)
            thread.start()
            thread.join(timeout)
            if thread.is_alive():
                logging.DEBUG(f"{func.__name__} timed out after {timeout} seconds")
                raise TimeoutException(
                    f"{func.__name__} timed out after {timeout} seconds"
                )
            if isinstance(result[0], Exception):
                raise result[0]
            return result[0]

        return wrapper

    return decorator


def setup_test(func):
    """Launch a test and handle its verdict"""

    def wrapper(*args, **kwargs):
        test = PicoHostAdapterDio(PORT_COM_DUT)
        func(test, *args, **kwargs)
        test.__del__()

    return wrapper


# ============= Main Functions =================

if __name__ == "__main__":
    help(__name__)
