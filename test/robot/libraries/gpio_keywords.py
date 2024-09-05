from API_PicoHostAdapterDio import PicoHostAdapterDio

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

def set_pin_mode(pin, mode):
    '''set the pin mode'''
    test.set_pin_mode(pin, mode)

def get_pin_mode(pin):
    '''return the pin mode'''
    return test.get_pin_mode(pin)['value']

# ================== Main ======================
if __name__ == '__main__':
    help(__name__)
