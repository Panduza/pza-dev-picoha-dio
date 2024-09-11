#!/usr/bin/env python3
"""API to control Pico Host Adapter Dio"""

import __future__

__author__  = 'Jason PUEL'
__date__    = "12 Aou 2024"

# ================== Imports ===================
import logging
import serial
import sliplib as sl
import api_dio_pb2 as dio

# ================== Variables =================


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

    def __picoha_dio_request(self,request_type:dio.RequestType,pin_num:int=None,pin_value:dio.PinValue=None) -> bool:
        """Send commend by serial COM"""
        picoha_dio_request = dio.PicohaDioRequest()
        picoha_dio_request.type = request_type
        picoha_dio_request.value = pin_value 
        picoha_dio_request.pin_num = pin_num 
        try:
            # Send/Write PicohaDioRequest in serial in binary using slip
            self.__serialPort.write(sl.encode(picoha_dio_request))
            logging.debug("Request: SUCCESS")
        except Exception as err:
            logging.error(f'Request: {err.args[1]}')

    def __picoha_dio_answer(self):
        """Wait answer on serial COM"""
        try:
            while 1:
                # Read data out of the buffer until a carraige return / new line is found
                serialString = self.__serialPort.readline()
                break
            picoha_dio_answer = dio.AnswerType()
            picoha_dio_answer.ParseFromString(sl.decode(serialString))
            logging.debug(picoha_dio_answer.ParseFromString(sl.decode(serialString)))
            if picoha_dio_answer.type != dio.AnswerType.SUCCESS :
                logging.warning("Answer: FAILURE")
            else:
                logging.debug("Answer: SUCCESS")
            return picoha_dio_answer
        except dio.ProtocolError :
            logging.error(dio.ProtocolError)
        except Exception as err:
            logging.error(f'{err.args[1]}')
    
    def is_connected(self)-> bool:
        '''Check if the serial port is open'''
        return self.__serialPort.is_open

    def ping_info(self):
        '''Get ping info'''
        self.__picoha_dio_request(dio.RequestType.PING)
        return self.__picoha_dio_answer().type

    def set_gpio_mode(self, pin:int, mode:dio.PinValue) -> dio.AnswerType:
        '''Set mode of pin in INPUT/OUTPUT'''
        self.__picoha_dio_request(dio.RequestType.SET_PIN_DIRECTION,pin,mode)
        return self.__picoha_dio_answer.type

    def set_gpio_value(self, pin:int, value:dio.PinValue) -> dio.AnswerType:
        '''Set value of pin as HIGH/LOW'''
        self.__picoha_dio_request(dio.RequestType.SET_PIN_VALUE,pin,value)
        return self.__picoha_dio_answer.type

    def get_gpio_mode(self, pin:int) -> dio.PinValue:
        '''Get mode of pin in INPUT/OUTPUT'''
        self.__picoha_dio_request(dio.RequestType.GET_PIN_DIRECTION,pin) 
        return self.__picoha_dio_answer.value

    def get_gpio_value(self, pin:int) -> dio.PinValue:
        '''Get value of pin as HIGH/LOW'''
        self.__picoha_dio_request(dio.RequestType.GET_PIN_VALUE,pin) 
        return self.__picoha_dio_answer.value


# ================== Main ======================
if __name__ == '__main__':
    help(PicoHostAdapterDio)