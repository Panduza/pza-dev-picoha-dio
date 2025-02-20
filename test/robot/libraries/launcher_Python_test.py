#!/usr/bin/env python3
"""
Launcher Python tests. 
Decorator to manage python test and useful function are describe here.
"""

import __future__

__author__ = "Jason PUEL"
__date__ = "12 Jan 2025"

# ================== Imports ===================

import threading
import functools
import logging

# Local imports
from platform_data import PORT_COM_DUT
from API_PicoHostAdapterDio import PicoHostAdapterDio

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
