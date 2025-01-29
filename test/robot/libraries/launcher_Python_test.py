#!/usr/bin/env python3
"""
Launcher Python tests. 
Decorator to manage python test and usefull fonction are discribe here.
"""

import __future__

__author__ = "Jason PUEL"
__date__ = "12 Jan 2025"

# ================== Imports ===================

import threading
import functools
import pydoc
import logging

# Local imports
from platform_data import PORT_COM_DUT
from API_PicoHostAdapterDio import PicoHostAdapterDio

# ================== Variables =================

_results = []

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


def test_launcher(func):
    """Launch a test and handle its verdict"""
    description = pydoc.render_doc(func).splitlines()[3][4:]

    def wrapper(*args, **kwargs):
        test = {"test": func.__name__, "description": description, "verdict": "ONGOING"}
        try:
            func(*args, **kwargs)
            test.update({"verdict": "PASS"})
        except TimeoutException as e:
            test.update({"error": e})
        except Exception as e:
            test.update({"verdict": "FAIL", "error": e})
        finally:
            _results.append(test)

    return wrapper


def setup_test(func):
    """Launch a test and handle its verdict"""

    def wrapper(*args, **kwargs):
        test = PicoHostAdapterDio(PORT_COM_DUT)
        func(test, *args, **kwargs)
        test.__del__()

    return wrapper


# ============ Utilities Fonctions =============


def results_md_chart():
    """Display result in chart on markdown format"""
    len_clo_0, len_clo_1, len_clo_2, len_clo_3 = 10, 45, 45, 5
    report = f'| {"Verdict":{len_clo_0}} | {"Tests Name":{len_clo_1}} | {"Description":{len_clo_2}} | Error |\n'
    report += f"| {'':-<{len_clo_0}} | {'':-<{len_clo_1}} | {'':-<{len_clo_2}} | {'':-<{len_clo_3}} |\n"
    for result in _results:
        report += (
            f'| {result["verdict"]:{len_clo_0}} | {result["test"]:{len_clo_1}} | {result["description"]:{len_clo_2}} | '
            + f'{result.get("error") if result.get("error") else f'{"":{len_clo_3}}'} |\n'
        )
    return report


def results_csv():
    """Display result in chart on CSV format"""
    report = "Verdict,Tests_Name,Description,Error\n"
    for result in _results:
        report += f'{result["verdict"]},{result["ticket"]},{result["description"]},{result.get("error") if result.get("error") else ""}\n'
    return report


def result_analyze():
    """Count PASS, FAIL, ONGOING 'verdict' in results"""
    total_test_run = len(_results)
    number_of_test_pass = 0
    number_of_test_ongoing = 0
    number_of_test_fail = 0

    for result in _results:
        if result["verdict"] == "PASS":
            number_of_test_pass += 1
        if result["verdict"] == "ONGOING":
            number_of_test_ongoing += 1
        if result["verdict"] == "FAIL":
            number_of_test_fail += 1

    return (
        total_test_run,
        number_of_test_pass,
        number_of_test_ongoing,
        number_of_test_fail,
    )


def print_results():
    """Use print fonction to display result in term."""
    total_test_run, number_of_test_pass, number_of_test_ongoing, number_of_test_fail = (
        result_analyze()
    )

    print("----------Results----------")
    print("- Detail\n")
    print(results_md_chart())
    print()
    print("- Abstract\n")
    print(f"    * {number_of_test_pass} tests PASS out of {total_test_run}")
    print(f"    * {number_of_test_fail} tests FAIL out of {total_test_run}")
    print(f"    * {number_of_test_ongoing} tests ONGOING out of {total_test_run}")
    print(f"Validated at {100*(number_of_test_pass)/total_test_run} %.\n")
    print("---------------------------")


# ============= Main Fonctions =================

if __name__ == "__main__":

    help(__name__)
