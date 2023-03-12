#!/usr/bin/env python

import _thread
import os
import re
import subprocess
import sys
import time
from typing import Tuple

import common

icon_fg = common.pink
icon_bg = common.black
icon_tr = "0xff"
text_fg = common.pink
text_bg = common.black
text_tr = "0xff"

icon_color = "^c" + str(icon_fg) + "^^b" + str(icon_bg) + str(icon_tr) + "^"
text_color = "^c" + str(text_fg) + "^^b" + str(text_bg) + str(text_tr) + "^"

DELAY_TIME = 1

filename = os.path.basename(__file__)
name = re.sub("\\..*", '', filename)

# 获取接口名
interface_name = subprocess.check_output("ip route | grep default | cut -d' ' -f5", shell=True).decode().strip()


def get_speed(val: int) -> str:
    if val < 1024:
        ret = "{:^8}".format(str(val) + "B")
    elif val < 1048576:
        ret = "{:^8}".format("{:.1f}".format(val / 1024) + "KB")
    else:
        ret = "{:^8}".format("{:.1f}".format(val / 1048576) + "MB")
    return ret


def getnet() -> Tuple[str, str]:
    rx_bytes_cur = 0
    cmd = "cat /sys/class/net/{}/statistics/rx_bytes".format(interface_name)
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    rx_bytes_string = result.stdout.decode('utf-8')
    for rx in rx_bytes_string.splitlines():
        rx_bytes_cur += int(rx)
    tx_positon = "~/.cache/rx_bytes"
    if not os.path.exists(tx_positon):
        os.system("touch " + tx_positon)
    cmd = "cat " + tx_positon
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    rx_bytes_pre = result.stdout.decode('utf-8').replace("\n", "")
    if rx_bytes_pre == "":
        rx_bytes_pre = 0
    rx_bytes = abs(int(rx_bytes_cur) - int(rx_bytes_pre))
    # write new rx_bytes_cur
    cmd = "echo " + str(rx_bytes_cur) + " > " + tx_positon
    os.system(cmd)

    tx_bytes_cur = 0
    cmd = "cat /sys/class/net/{}/statistics/tx_bytes".format(interface_name)
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    tx_bytes_string = result.stdout.decode('utf-8')
    for tx in tx_bytes_string.splitlines():
        tx_bytes_cur += int(tx)
    tx_positon = "~/.cache/tx_bytes"
    if not os.path.exists(tx_positon):
        os.system("touch " + tx_positon)
    cmd = "cat " + tx_positon
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    tx_bytes_pre = result.stdout.decode('utf-8').replace("\n", "")
    if tx_bytes_pre == "":
        tx_bytes_pre = 0
    tx_bytes = abs(int(tx_bytes_cur) - int(tx_bytes_pre))
    # write new tx_bytes_cur
    cmd = "echo " + str(tx_bytes_cur) + " > " + tx_positon
    os.system(cmd)

    send_string = str(get_speed(tx_bytes))
    recv_string = str(get_speed(rx_bytes))
    # print(send_string)
    # print(recv_string)
    return " " + send_string, " " + recv_string


def update(loop=False, set_root=True):
    while True:
        text = ""
        for string in getnet():
            text += string
        txt = "^s{}^{} {} ".format(name, text_color, text)
        common.write_to_file(txt + "\n", str(name))
        if not loop:
            if set_root:
                os.system("xsetroot -name '" + str(txt) + "'")
            break
        time.sleep(DELAY_TIME)


def update_thread():
    _thread.start_new_thread(update, (False, False))


def click(string=''):
    match string:
        case 'L':
            pass
        case 'M':
            pass
        case 'R':
            pass
        case 'U':
            pass
        case 'D':
            pass
        case _:
            pass


def notify():
    pass


if __name__ == "__main__":
    print(interface_name)
    if len(sys.argv) > 1:
        if sys.argv[1] == "update":
            pass
        else:
            click(sys.argv[1])
            update(set_root=False)
    else:
        update()
