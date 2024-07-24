import serial
import logging
import serial.tools.list_ports

from robot.libraries.BuiltIn import BuiltIn


###############################################################################

def I_list_all_the_USB_devices():
    devices_object = []
    devices = serial.tools.list_ports.comports()
    for device in devices:
        devices_object.append({
            "port_name": device.device,
            "pid": device.pid,
            "vid": device.vid,
            "serial_number": device.serial_number
        })
    logging.debug("All devices: ", devices_object)
    BuiltIn().set_global_variable('${ALL_DEVICES}', devices_object)

###############################################################################

def find_port_name_from_USB_ids(vid, pid):
    devices = serial.tools.list_ports.comports()
    for device in devices:
        if device.vid == vid and device.pid == pid:
            return device.device
    return None

###############################################################################

