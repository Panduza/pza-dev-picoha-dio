#!/usr/bin/env python3
"""This file link RobotFramework Keywords we python API"""
import logging

# local imports
import api_dio_pb2 as dio
from API_PicoHostAdapterDio import PicoHostAdapterDio
#from ..platform.RaspberryPico import platform_data as platform

# ================== Keys World ================

def test_that_nothing_is_good_too():
    print("This is a test that does nothing, but it's good too.")
    print("You should be able to see this inside your test report.")

# ==============================================
test = PicoHostAdapterDio("COM4")

def is_connected():
    try:
        return "true" if test.is_connected() else "false"
    except:
        return "false"
    
def ping():
    '''send Ping frame'''
    try :
        return "true" if (test.ping_info() == dio.AnswerType.SUCCESS) else "false"
    except:
        return "false"

def set_gpio_direction(gpio:int, direction:dio.PinValue):
    '''set gpio direction'''
    if direction == 'INPUT':
        err = test.set_gpio_direction(gpio, dio.PinValue.INPUT)
    elif direction == 'OUTPUT':
        err = test.set_gpio_direction(gpio, dio.PinValue.OUTPUT)
    else:
        logging.warning(f"This is not Direction value : {direction}")
        err = dio.AnswerType.FAILURE
    
    return "SUCCESS" if err == dio.AnswerType.SUCCESS else "FAILURE"

def get_gpio_direction(gpio:int):
    '''return the gpio direction'''
    direction = test.get_gpio_direction(gpio)
    if direction == dio.PinValue.INPUT:
        return 'INPUT'
    elif direction == dio.PinValue.OUTPUT:
        return 'OUTPUT'
    else :
        logging.warning(f"This is not Direction value : {direction}")
        return None

def set_gpio_value(gpio:int, value):
    '''set the gpio direction'''
    if test.set_gpio_value(gpio, value) == dio.AnswerType.SUCCESS:
        return ("SUCCESS")
    else :
        return ("FAILURE")

def get_gpio_value(gpio:int):
    '''return the gpio direction'''
    value = test.get_gpio_value(gpio)
    if value == dio.PinValue.LOW:
        return 'LOW'
    elif value == dio.PinValue.HIGH:
        return 'HIGH'
    else :
        logging.warning("this is not value")
        return None

# ==============================================

def get_comp_gpio(gpio:int):
    '''get paired GPIO'''
    #if gpio not in platform.GPIO_USABLE:
    #    logging.warning('This gpio is not usable.')
    #    return "FAILURE"
    if gpio==23 or gpio==24:
        return "FAILURE"
    elif gpio == 25 :
        logging.warning('GPIO 25 is builtin LED')
        return "FAILURE"
    elif gpio==22: return 26
    elif gpio==26: return 22
    elif gpio==27: return 28
    elif gpio==28: return 27
    else :
        return gpio-1 if gpio % 2 else gpio+1
     
def set_gpio_direction_and_his_comp(gpio:int, direction:dio.PinValue):
    '''set gpio direction AND complementary gpio direction according to shematics'''
    comp_gpio = get_comp_gpio(gpio)
    logging.debug(f'{gpio} is connected to {comp_gpio}')
    if direction == 'INPUT':
        err1 = test.set_gpio_direction(gpio, dio.PinValue.INPUT)
        err2 = test.set_gpio_direction(comp_gpio, dio.PinValue.OUTPUT)
    elif direction == 'OUTPUT':
        err1 = test.set_gpio_direction(gpio, dio.PinValue.OUTPUT)
        err2 = test.set_gpio_direction(comp_gpio, dio.PinValue.INPUT)
    else:
        logging.warning(f"This is not Direction value : {direction}")
        err1,err2 = dio.AnswerType.FAILURE
    if err1 == dio.AnswerType.SUCCESS and err2 == dio.AnswerType.SUCCESS:
        return ("SUCCESS")
    else :
        return ("FAILURE")
    
def get_gpio_value_comp(gpio:int):
    return get_gpio_value(get_comp_gpio(gpio))

def set_gpio_value_comp(gpio:int, value):
    return set_gpio_value(get_comp_gpio(gpio), value)

# ================== Main ======================
if __name__ == '__main__':
    help(__name__)
