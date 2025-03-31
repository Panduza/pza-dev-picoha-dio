"""
This file describe Raspberry Pico test bench setup.
"""

import logging, os
from configparser import ConfigParser


_init_file = os.path.join(os.path.dirname(__file__), "config_file.ini")
PORT_COM_DUT: str = ""
if os.path.exists(_init_file):
    logging.info(f"Load config from :{_init_file}")
    _config = ConfigParser()
    _config.read(_init_file)
    PORT_COM_DUT = (
        _config.get("General", "PortCOM")
        if _config.has_option("General", "PortCOM")
        else ""
    )
else:
    logging.info(f"we didn't found: {_init_file}.")

BUILTIN_LED = 25
GPIO_UART = [0, 1]
GPIO_USABLE = [
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    26,
    27,
    28,
]


def get_comp_gpio(gpio: int):
    """
       Get paired GPIO

       # Raspberry Pico test bench Setup
       To simplified the validation process GPIO are connected in pair.
               ┌──────┐--┌──────┐
     ── GPIO0  | 1    └──┘   40 |  VBUS
     ── GPIO1  | 2    USB    39 |  VSYS
          GND  | 3           38 |  GND
    ┌─  GPIO2  | 4           37 |  3V3_EN
    └─  GPIO3  | 5  [GPIO25] 36 |  3V3
    ┌─  GPIO4  | 6           35 |
    └─  GPIO5  | 7           34 |  GPIO28  ─┐
          GND  | 8   ┌    ┐  33 |  GND      |
    ┌─  GPIO6  | 9    PICO   32 |  GPIO27  ─┘
    └─  GPIO7  | 10  └    ┘  31 |  GPIO26  ─┐
    ┌─  GPIO8  | 11          30 |  Run      |
    └─  GPIO9  | 12          29 |  GPIO22  ─┘
          GND  | 13          28 |  GND
    ┌─ GPIO10  | 14          27 |  GPIO21  ─┐
    └─ GPIO11  | 15          26 |  GPIO20  ─┘
    ┌─ GPIO12  | 16          25 |  GPIO19  ─┐
    └─ GPIO13  | 17          24 |  GPIO18  ─┘
          GND  | 18          23 |  GND
    ┌─ GPIO14  | 19          22 |  GPIO17  ─┐
    └─ GPIO15  | 20   DEBUG  21 |  GPIO16  ─┘
               └───────┴┴┴──────┘

    """
    if gpio == BUILTIN_LED:
        raise ValueError("GPIO 25 is builtin LED: there is no paired GPIO to it.")
    elif gpio in GPIO_UART:
        raise ValueError("GPIO_UART are not usable: reserved for debugging propose")
    elif gpio not in GPIO_USABLE:
        raise ValueError("This gpio is not usable on this Setup.")

    elif gpio == 22:
        return 26
    elif gpio == 26:
        return 22

    elif gpio == 27:
        return 28
    elif gpio == 28:
        return 27

    else:
        return gpio - 1 if gpio % 2 else gpio + 1


# ================== Main ======================
if __name__ == "__main__":
    help(__name__)
