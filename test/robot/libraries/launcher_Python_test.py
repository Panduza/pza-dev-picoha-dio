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


# ================== Variables =================

_results = []


# ================== Class =====================


class TimeoutException(Exception):
    # Do nothing only handle timeout error
    pass


class TestFailException(Exception):
    pass


# ================== Decorator =================


def timeout_wrapper(timeout=60):
    """
    Launch a fonction and break it on timeout.
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
    ticket_name = pydoc.render_doc(func).splitlines()[3][4:]

    def wrapper(*args, **kwargs):
        test = {"test": func.__name__, "ticket": ticket_name, "verdict": "ONGOING"}
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


# ============ Utilities Fonctions =============


def results_md_chart():
    """Display result in chart on markdown format"""
    report = f'| {"Verdict":10} | {"Ticket Name":45} | {"Tests Name":40} | Error |\n'
    report += f"| {'':-<10} | {'':-<45} | {'':-<40} | ----- |\n"
    for result in _results:
        report += f'| {result["verdict"]:10} | {result["ticket"]:45} | {result["test"]:40} | {result.get("error") if result.get("error") else f'{"":5}'} |\n'
    return report


def results_csv():
    """Display result in chart on CSV format"""
    report = "Verdict,Ticket_Name,Tests_Name,Error\n"
    for result in _results:
        report += f'{result["verdict"]},{result["ticket"]},{result["test"]},{result.get("error") if result.get("error") else ""}\n'
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
