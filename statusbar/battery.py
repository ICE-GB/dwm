#!/usr/bin/env python

import _thread
import os
import re
import subprocess
import sys
import time

import common

icon_fg = common.pink
icon_bg = common.black
icon_tr = "0xff"
text_fg = common.pink
text_bg = common.black
text_tr = "0xff"

icon_color = "^c" + str(icon_fg) + "^^b" + str(icon_bg) + str(icon_tr) + "^"
text_color = "^c" + str(text_fg) + "^^b" + str(text_bg) + str(text_tr) + "^"
DELAY_TIME = 3

filename = os.path.basename(__file__)
name = re.sub("\\..*", '', filename)


def get_battery_status_from_acpi():
    status = subprocess.check_output(["acpi"]).decode("utf-8").strip()
    if not status:
        return 100, False
    percent = int(re.search(r"\d+(?=%)", status).group())
    charging = "+" in status
    return percent, charging


def get_battery_status():
    battery_text, charge_sta = get_battery_status_from_acpi()

    if charge_sta:
        bat_icon = ""
    elif battery_text < 25:
        bat_icon = ""
    elif battery_text < 50:
        bat_icon = ""
    elif battery_text < 75:
        bat_icon = ""
    else:
        bat_icon = ""
    return bat_icon, battery_text


def update(loop=False, set_root=True):
    while True:
        icon, text = get_battery_status()
#         txt = "^s{}^{} {} {}{} ".format(name, icon_color, icon, text_color, text)
        txt = "^s{}^{} {} ".format(name, icon_color, icon)
        common.write_to_file(txt + "\n", str(name))
        if not loop:
            if set_root:
                os.system("xsetroot -name '" + str(txt) + "'")
            break
        time.sleep(DELAY_TIME)


def update_thread():
    _thread.start_new_thread(update, (False, False))


def notify():
    pass


def click(button=''):
    match button:
        case 'L':
            os.system("kcmshell5 kcm_powerdevilprofilesconfig")
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


if __name__ == "__main__":
    if len(sys.argv) > 1:
        if sys.argv[1] == "update":
            pass
        else:
            click(sys.argv[1])
            update(set_root=False)
    else:
        update()
