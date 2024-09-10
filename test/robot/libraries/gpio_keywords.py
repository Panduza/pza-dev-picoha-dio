from API_PicoHostAdapterDio import PicoHostAdapter
import logging
import api_dio_pb2 as dio
import ..platform.RaberryPico.platform_data
# ================== Keys World ================

def test_that_nothing_is_good_too():
    print("This is a test that does nothing, but it's good too.")
    print("You should be able to see this inside your test report.")

# ==============================================
test = PicoHostAdapterDio("COM3")

def is_connected():
    if test.is_connected():
        return "true"
    else:
        return "false"
    
def ping():
    test.ping_info()

def set_gpio_mode(gpio:int, mode):
    '''set the gpio mode'''
    comp_gpio = get_comp_gpio(gpio)
    if mode.uper() == 'INPUT':
        err1 = test.set_gpio_mode(gpio, dio.PinValue.INPUT)
        err2 = test.set_gpio_mode(comp_gpio, dio.PinValue.OUTPUT)
    else:
        err1 = test.set_gpio_mode(gpio, dio.PinValue.OUTPUT)
        err2 = test.set_gpio_mode(comp_gpio, dio.PinValue.INPUT)
    if err1 == dio.AnswerType.SUCCESS and err2 == dio.AnswerType.SUCCESS:
        logging.debug("SUCCESS")
        return ("SUCCESS")
    else :
        logging.debug("FAILURE")
        return ("FAILURE")

def get_gpio_mode(gpio:int) -> str :
    '''return the gpio mode'''
    direction = test.get_gpio_mode(gpio)
    if direction == dio.PinValue.INPUT:
        return 'INPUT'
    elif direction == dio.PinValue.OUTPUT:
        return 'OUTPUT'
    else :
        logging.warning("this is not Direction value")
        return None

def set_gpio_value(gpio:int, value):
    '''set the gpio mode'''
    if test.set_gpio_value(gpio, value) == dio.AnswerType.SUCCESS:
        logging.debug("SUCCESS")
        return ("SUCCESS")
    else :
        logging.debug("FAILURE")
        return ("FAILURE")

def get_gpio_value(gpio:int):
    '''return the gpio mode'''
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
    if gpio == 25 :
        logging.warning('GPIO 25 is builtin LED')
        return -1
    elif gpio not in GPIO_USABLE:
        logging.warning('this gpio is not usable.')
        return -1
    else :
        return gpio+1 if not gpio==24 else gpio+2
    
def get_gpio_value_comp(gpio:int):
    return get_gpio_value(get_comp_gpio(gpio))

def set_gpio_value_comp(gpio:int):
    return set_gpio_value(get_comp_gpio(gpio))

# ================== Main ======================
if __name__ == '__main__':
    help(__name__)
