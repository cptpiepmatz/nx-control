import struct
import nxbt
import socket

HOST = "0.0.0.0"
PORT = 5656

nx = nxbt.Nxbt()

controller_index = nx.create_controller(
    nxbt.PRO_CONTROLLER,
    reconnect_address=nx.get_switch_addresses())
nx.wait_for_connection(controller_index)

print("Controller connected")

def parse_press_button(data):
    return {
        # Sticks
        "L_STICK": {
            "PRESSED": data[2] != 0,
            "X_VALUE": struct.unpack('>f', data[3:7])[0] * 100,
            "Y_VALUE": struct.unpack('>f', data[7:11])[0] * 100,
            # Keyboard position calculation values
            "LS_UP": False,
            "LS_LEFT": False,
            "LS_RIGHT": False,
            "LS_DOWN": False
        },
        "R_STICK": {
            "PRESSED": data[11] != 0,
            "X_VALUE": struct.unpack('>f', data[12:16])[0] * 100,
            "Y_VALUE": struct.unpack('>f', data[16:20])[0] * 100,
            # Keyboard position calculation values
            "RS_UP": False,
            "RS_LEFT": False,
            "RS_RIGHT": False,
            "RS_DOWN": False
        },
        # Dpad
        "DPAD_UP": data[0] & (1 << 3) != 0,
        "DPAD_LEFT": data[0] & (1 << 2) != 0,
        "DPAD_RIGHT": data[0] & (1 << 1) != 0,
        "DPAD_DOWN": data[0] & (1 << 0) != 0,
        # Triggers
        "L": data[1] & (1 << 7) != 0,
        "ZL": data[1] & (1 << 6) != 0,
        "R": data[1] & (1 << 5) != 0,
        "ZR": data[1] & (1 << 4) != 0,
        # Joy-Con Specific Buttons
        "JCL_SR": False,
        "JCL_SL": False,
        "JCR_SR": False,
        "JCR_SL": False,
        # Meta buttons
        "PLUS": data[1] & (1 << 3) != 0,
        "MINUS": data[1] & (1 << 2) != 0,
        "HOME": data[1] & (1 << 1) != 0,
        "CAPTURE": data[1] & (1 << 0) != 0,
        # Buttons
        "Y": data[0] & (1 << 4) != 0,
        "X": data[0] & (1 << 5) != 0,
        "B": data[0] & (1 << 6) != 0,
        "A": data[0] & (1 << 7) != 0
    }

with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s:
    s.bind((HOST, PORT))
    print("UDPServer Waiting for client on port", PORT)
    while True:
        data, addr = s.recvfrom(32)
        print("Got data from", addr)
        print(data)
        if (data[0] == 0):
            print("None")
        elif(data[0] == 1):
            print("Create Controller")
        elif(data[0] == 2):
            print("Press Button")
            packet = parse_press_button(data[1:])
            print(packet)
            nx.set_controller_input(controller_index, packet)
        else:
            print("Error")
