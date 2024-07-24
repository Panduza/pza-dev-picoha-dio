import logging
import serial
from usb_keywords import find_port_name_from_USB_ids
from robot.libraries.BuiltIn import BuiltIn


def a_serial_connection_to_the_device_is_opened():
    
    port_name = find_port_name_from_USB_ids(0x16c0, 0x05E1)
    logging.info(f"Try to open port: {port_name}")

    ser = serial.Serial(port_name, 9600, timeout=1)
    ser.open()

    BuiltIn().set_global_variable('${SERIAL}', ser)


