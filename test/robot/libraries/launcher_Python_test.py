#!/usr/bin/env python3
"""Launcher Python tests"""

import __future__

__author__ = "Jason PUEL"
__date__ = "12 Jan 2025"

# ================== Imports ===================

import threading
import functools


# ================== Variables =================

_results = []


# ================== Class =====================
class TimeoutException(Exception):
    # Do nothing only handle timeout error
    pass


class TestFailException(Exception):
    pass


# ================== Decorator =================


def timeout_wrapper(timeout):
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
                raise TimeoutException(
                    f"{func.__name__} timed out after {timeout} seconds"
                )
            if isinstance(result[0], Exception):
                raise result[0]
            return result[0]

        return wrapper

    return decorator


def test_launcher(func):
    def wrapper(*args, **kwargs):
        test = {"test": func.__name__, "status": "ONGOING"}
        try:
            func(*args, **kwargs)
            test.update({"status": "PASS"})
        except TimeoutException as e:
            test.update({"error": e})
        except Exception as e:
            test.update({"status": "FAIL", "error": e})
        finally:
            _results.append(test)

    return wrapper


# ============ Utilities Fonctions =============


def results_md_chart():
    report = f'| {"Status":10} | {"Tests Name":40} | Error |\n'
    report += f"| {'':-<10} | {'':-<40} | ----- |\n"
    for result in _results:
        report += f'| {result["status"]:10} | {result["test"]:40} | {result.get("error") if result.get("error") else f'{"":5}'} |\n'
    return report


def results_csv():
    report = "Status,Tests_Name,Error\n"
    for result in _results:
        report += f'{result["status"]},{result["test"]},{result.get("error") if result.get("error") else ""}\n'
    return report


def print_results():
    total_test_run = len(_results)
    number_of_test_pass = 0
    number_of_test_ongoing = 0
    number_of_test_fail = 0

    for result in _results:
        if result["status"] == "PASS":
            number_of_test_pass += 1
        if result["status"] == "ONGOING":
            number_of_test_ongoing += 1
        if result["status"] == "FAIL":
            number_of_test_fail += 1

    print("----------Results----------")
    print("- Detail\n")
    print(results_md_chart())
    print()
    print("- Abstract\n")
    print(f"    * {number_of_test_pass} tests PASS out of {total_test_run}")
    print(f"    * {number_of_test_fail} tests FAIL out of {total_test_run}")
    print(f"    * {number_of_test_ongoing} tests ONGOING out of {total_test_run}")
    print(f"Validated a {100*(number_of_test_pass)/total_test_run} %\n")
    print("---------------------------")


# ============= Main Fonctions =================

if __name__ == "__main__":

    help(__name__)
