"""API to control Pico Host Adapter Dio"""

import __future__

__author__  = 'Jason PUEL'
__date__    = "12 Aou 2024"

# ================== Imports ===================
import logging
import re
import serial
from enum import Enum

# ================== Variables =================


# ================== Enums =====================
class RequestType(Enum):
    PING = 0
    SET_PIN_DIRECTION = 1
    SET_PIN_VALUE = 2
    GET_PIN_DIRECTION = 3
    GET_PIN_VALUE = 4
    def __str__(self):
        return self.name

class PinValue(Enum):
    LOW = 0
    HIGH = 1
    INPUT = 2
    OUTPUT = 3
    def __str__(self):
        return self.name

class AnswerType(Enum):
    SUCCESS = 0
    FAILURE = 1
    def __str__(self):
        return self.name


# ================== Class =====================
class PicoHostAdapterDio():
    def __init__(self,serial_COM:str,baudrate:int=9600,bytesize:int=8,timeout:int=2):
        self.__serialPort = serial.Serial(
            port=serial_COM, 
            baudrate=baudrate, 
            bytesize=bytesize, 
            timeout=timeout, 
            stopbits=serial.STOPBITS_ONE
            )

    def __del__(self):
        try:
            if self.is_connected():
                self.__serialPort.close()
        except Exception as err:
            logging.error(f'{err.args[1]}')

    def __picoha_dio_request(self,request_type:RequestType,pin_num:int=None,pin_value:PinValue=None) -> bool:
        """Send commend by serial COM"""
        try:
            # Send/Write PicohaDioRequest in serial in binary 
            self.__serialPort.write(b"message PicohaDioRequest {" +\
                        f"RequestType type = {request_type}; " +\
                        f"uint32 pin_num = {pin_num}; " +\
                        f"PinValue value = {pin_value}; " +\
                        "}\r\n")
            logging.debug("Request: SUCCESS")
        except Exception as err:
            logging.error(f'Request: {err.args[1]}')

    def __picoha_dio_answer(self):
        """Wait answer on serial COM"""
        try:
            while 1:
                # Read data out of the buffer until a carraige return / new line is found
                serialString = self.__serialPort.readline().decode("Ascii")
                break
            serialString = serialString.replace("optional ", "")
            fields = re.findall(r'(\w+)\s(\w+)\s=\s(\d+);', serialString)
            picoha_dio_answer = {field[1]: int(field[2]) for field in fields}
            if picoha_dio_answer["type"] != AnswerType.SUCCESS.value :
                logging.warning("Answer: FAILURE")
            else:
                logging.debug("Answer: SUCCESS")
            return picoha_dio_answer
        except Exception as err:
            logging.error(f'{err.args[1]}')
    
    def is_connected(self)-> bool:
        '''Check if the serial port is open'''
        return self.__serialPort.is_open

    def ping_info(self):
        '''Get ping info'''
        #TODO: sort and return the 'ping' value
        self.__picoha_dio_request(RequestType.PING.value)
        return self.__picoha_dio_answer()

    def set_pin_mode(self, pin:int, mode:PinValue) -> AnswerType:
        '''Set mode of pin in INPUT/OUTPUT'''
        self.__picoha_dio_request(RequestType.SET_PIN_DIRECTION.value,pin,mode)
        return AnswerType(self.__picoha_dio_answer()['type'])

    def set_pin_value(self, pin:int, value:PinValue) -> AnswerType:
        '''Set value of pin as HIGH/LOW'''
        self.__picoha_dio_request(RequestType.SET_PIN_VALUE.value,pin,value)
        return AnswerType(self.__picoha_dio_answer()['type'])

    def get_pin_mode(self, pin:int) -> PinValue:
        '''Get mode of pin in INPUT/OUTPUT'''
        self.__picoha_dio_request(RequestType.GET_PIN_DIRECTION.value,pin) 
        return PinValue(self.__picoha_dio_answer()['value'])

    def get_pin_value(self, pin:int) -> PinValue:
        '''Get value of pin as HIGH/LOW'''
        self.__picoha_dio_request(RequestType.GET_PIN_VALUE.value,pin) 
        return PinValue(self.__picoha_dio_answer()['value'])


# ================== Main ======================
if __name__ == '__main__':
    help(PicoHostAdapterDio)