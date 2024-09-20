#!/usr/bin/env python3
"""API to control Pico Host Adapter Dio"""

import __future__

__author__  = 'Jason PUEL'
__date__    = "12 Aou 2024"

# ================== Imports ===================
import logging
import serial, os
import sliplib as sl
import api_dio_pb2 as dio
from google.protobuf.json_format import MessageToJson, MessageToDict

# ================== Variables =================


# ================== Fonctions =================
def setup_logging(loggingLevel=logging.INFO,
        savelog=False,
        logpath=os.path.dirname(__file__),
        logfile='Auto_Report.log') -> logging:
    """Start recording logs on consol_log and stream it on the terminal"""

    logformat = '%(asctime)s:%(msecs)03d %(levelname)s - %(funcName)s: %(message)s'
    dateformat = "%Y/%m/%d %H:%M:%S"
    
    if savelog:
        logfullpath = os.path.join(logpath, logfile)
        from sys import stdout
        # When you use a logger with save file option, you need an stdout handler to display it in prompt
        logging.basicConfig(
            format=logformat, datefmt=dateformat,
            level=loggingLevel, 
            handlers=[logging.FileHandler(logfullpath),
                      logging.StreamHandler(stdout)])
    else:
        logging.basicConfig(format=logformat, datefmt=dateformat, level=loggingLevel)
    logger = logging.getLogger(__name__)
    return logger


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
            logging.error(f'{err}')

    def __picoha_dio_request(self,request_type:dio.RequestType,pin_num:int=None,pin_value:dio.PinValue=None) -> bool:
        """Send commend by serial COM"""
        picoha_dio_request = dio.PicohaDioRequest()
        picoha_dio_request.type = request_type
        if pin_value : picoha_dio_request.value = pin_value 
        if pin_num : picoha_dio_request.pin_num = pin_num
        logging.debug(MessageToDict(picoha_dio_request))
        try:
            # Send/Write PicohaDioRequest in serial in binary using slip
            self.__serialPort.write(sl.encode(picoha_dio_request.SerializeToString())+sl.END)
        except TypeError as err :
            logging.error(f'TypeError: {err}')
        except AttributeError as err :
            logging.error(f'AttributeError: {err}')
        except Exception as err:
            logging.error(f'Request Error: {err}')

    def __picoha_dio_answer(self):
        """Wait answer on serial COM"""
        try:
            while 1:
                # Read data out of the buffer until a carraige return / new line is found
                serialString = self.__serialPort.read(100)
                break 
            picoha_dio_answer = dio.PicohaDioAnswer()
            picoha_dio_answer.ParseFromString(sl.decode(serialString))
            logging.debug(MessageToDict(picoha_dio_answer))
            return picoha_dio_answer
        except Exception as err:
            logging.error(err)
    
    def is_connected(self)-> bool:
        '''Check if the serial port is open'''
        return self.__serialPort.is_open

    def ping_info(self):
        '''Get ping info'''
        self.__picoha_dio_request(dio.RequestType.PING)
        return self.__picoha_dio_answer().value

    def set_gpio_mode(self, pin:int, mode:dio.PinValue) -> dio.AnswerType:
        '''Set mode of pin in INPUT/OUTPUT'''
        self.__picoha_dio_request(dio.RequestType.SET_PIN_DIRECTION, pin, mode)
        return self.__picoha_dio_answer().type

    def set_gpio_value(self, pin:int, value:dio.PinValue) -> dio.AnswerType:
        '''Set value of pin as HIGH/LOW'''
        self.__picoha_dio_request(dio.RequestType.SET_PIN_VALUE,pin,value)
        return self.__picoha_dio_answer().type

    def get_gpio_mode(self, pin:int) -> dio.PinValue:
        '''Get mode of pin in INPUT/OUTPUT'''
        self.__picoha_dio_request(dio.RequestType.GET_PIN_DIRECTION,pin) 
        return self.__picoha_dio_answer().value

    def get_gpio_value(self, pin:int) -> dio.PinValue:
        '''Get value of pin as HIGH/LOW'''
        self.__picoha_dio_request(dio.RequestType.GET_PIN_VALUE,pin) 
        return self.__picoha_dio_answer().value


# ================== Main ======================
if __name__ == '__main__':
  
    help(PicoHostAdapterDio)
    '''
    ## Exemple:
    import time
    # Setup
    logger = setup_logging(loggingLevel = logging.DEBUG)

    test = PicoHostAdapterDio("COM4")
    test.ping_info()

    test.set_gpio_mode(pin=2,mode=dio.PinValue.OUTPUT)
    test.get_gpio_mode(pin=2)

    test.set_gpio_mode(pin=3,mode=dio.PinValue.INPUT)
    test.get_gpio_mode(pin=3)

    # Main
    for i in range(0,4,1):
        print()
        time.sleep(0.5)
        test.set_gpio_value(pin=2,value=1-i%2)
        time.sleep(0.5)
        test.get_gpio_value(pin=3)
    '''