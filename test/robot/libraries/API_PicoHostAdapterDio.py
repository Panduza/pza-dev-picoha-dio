#!/usr/bin/env python
"""API to control Pico Host Adapter Dio"""

import __future__

__author__ = "Jason PUEL"
__date__ = "12 Aou 2024"

# ================== Imports ===================
import logging
import serial, os
import sliplib as sl
from google.protobuf.json_format import MessageToDict
from robot.api.deco import library, keyword

# local imports
import api_dio_pb2 as dio

# ================== Variables =================


# ================== Functions =================
def setup_logging(
    logging_level=logging.INFO,
    log_save=False,
    log_path=os.path.dirname(__file__),
    log_file="Auto_Report.log",
) -> logging:
    """Start recording logs on consol_log and stream it on the terminal"""

    _log_format = "%(asctime)s:%(msecs)03d %(levelname)s - %(funcName)s: %(message)s"
    _dateformat = "%Y/%m/%d %H:%M:%S"

    if log_save:
        log_full_path = os.path.join(log_path, log_file)
        from sys import stdout

        # When you use a logger with save file option, you need an stdout handler to display it in prompt
        logging.basicConfig(
            format=_log_format,
            datefmt=_dateformat,
            level=logging_level,
            handlers=[
                logging.FileHandler(log_full_path),
                logging.StreamHandler(stdout),
            ],
        )
    else:
        logging.basicConfig(
            format=_log_format, datefmt=_dateformat, level=logging_level
        )
    logger = logging.getLogger(__name__)
    return logger


# ================== Class =====================
@library
class PicoHostAdapterDio(serial.Serial):
    """Main API class to control Pico Host Adapter Dio"""

    def __init__(
        self,
        port: str = None,
        baudrate: int = 9600,
        bytesize: int = 8,
        timeout: int = 2,
    ):
        super().__init__(
            port=port,
            baudrate=baudrate,
            bytesize=bytesize,
            timeout=timeout,
            stopbits=serial.STOPBITS_ONE,
        )

    def __del__(self):
        """Close serial port"""
        self.close()

    def __picoha_dio_request(
        self,
        request_type: dio.RequestType,
        pin_num: int = None,
        pin_value: dio.PinValue = None,
    ):
        """Send command by serial COM"""
        picoha_dio_request = dio.PicohaDioRequest()
        picoha_dio_request.type = request_type
        if pin_value:
            picoha_dio_request.value = pin_value
        if pin_num:
            picoha_dio_request.pin_num = pin_num
        logging.debug(MessageToDict(picoha_dio_request))
        try:
            # Send/Write PicohaDioRequest in serial in binary using slip
            self.write(sl.encode(picoha_dio_request.SerializeToString()) + sl.END)
        except Exception as err:
            logging.error(f"{err}")
            raise PicoHostAdapterDio(err)

    def __picoha_dio_answer(self) -> dio.PicohaDioAnswer:
        """Wait answer on serial COM"""
        try:
            # Read data out of the buffer until a carriage return / new line is found
            serialString = self.read(100)
            picoha_dio_answer = dio.PicohaDioAnswer()
            if len(serialString) == 0:
                logging.warning("Timeout: no Data received")
                picoha_dio_answer.type = dio.AnswerType.FAILURE
                return picoha_dio_answer
            picoha_dio_answer.ParseFromString(sl.decode(serialString))
            logging.debug(MessageToDict(picoha_dio_answer))
            return picoha_dio_answer
        except Exception as err:
            logging.error(err)
            raise PicoHostAdapterDio(err)

    @keyword
    def setup_context(self, port: str):
        self.__init__(port)

    @keyword
    def close_context(self):
        self.__del__()

    # --- Commend and Keyword ---
    @keyword
    def ping_info(self) -> dio.PicohaDioAnswer:
        """
        Get ping info: 0 => no problem
        return :
            type : Status of the command
        """
        self.__picoha_dio_request(dio.RequestType.PING)
        return self.__picoha_dio_answer().type

    @keyword
    def set_gpio_direction(
        self, gpio: int, direction: dio.PinValue
    ) -> dio.PicohaDioAnswer:
        """
        Set direction of pin in INPUT/OUTPUT
        return :
            type : Status of the command
        """
        self.__picoha_dio_request(dio.RequestType.SET_PIN_DIRECTION, gpio, direction)
        return self.__picoha_dio_answer().type

    @keyword
    def set_gpio_value(self, gpio: int, value: dio.PinValue) -> dio.PicohaDioAnswer:
        """
        Set value of gpio as HIGH/LOW
        return :
            type : Status of the command
        """
        self.__picoha_dio_request(dio.RequestType.SET_PIN_VALUE, gpio, value)
        return self.__picoha_dio_answer().type

    @keyword
    def get_gpio_direction(self, gpio: int) -> dio.PicohaDioAnswer:
        """
        Get direction of gpio in INPUT/OUTPUT
        return :
            type : Status of the command
            value : value of GPIO
        """
        self.__picoha_dio_request(dio.RequestType.GET_PIN_DIRECTION, gpio)
        return self.__picoha_dio_answer()

    @keyword
    def get_gpio_value(self, gpio: int) -> dio.PicohaDioAnswer:
        """
        Get value of gpio as HIGH/LOW
        return :
            type : Status of the command
            value : value of GPIO
        """
        self.__picoha_dio_request(dio.RequestType.GET_PIN_VALUE, gpio)
        return self.__picoha_dio_answer()


# ================== Main ======================
if __name__ == "__main__":
    """
    ## Example:
    import time

    # Setup
    setup_logging(logging_level=logging.DEBUG)

    test = PicoHostAdapterDio("COM5")
    test.ping_info()

    test.set_gpio_direction(gpio=2, direction=dio.PinValue.OUTPUT)
    dir_of_2 = test.get_gpio_direction(gpio=2).value

    test.set_gpio_direction(gpio=3, direction=dio.PinValue.INPUT)
    dir_of_3 = test.get_gpio_direction(gpio=3).value

    # Main
    for i in range(0, 4, 1):
        print()
        time.sleep(0.5)
        test.set_gpio_value(gpio=2, value=1 - i % 2)
        time.sleep(0.5)
        test.get_gpio_value(gpio=3).value
    """
    help(PicoHostAdapterDio)
