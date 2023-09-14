#!/usr/bin/env python

import _thread
import os
import re
import subprocess
import sys
import time

import psutil

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


def update(loop=False, set_root=True):
    while True:
        cpu_usage = int(psutil.cpu_percent())
        if cpu_usage > 50:
            icon = ""
        else:
            icon = ""
        cpu_usage = "{:<3}".format(str(cpu_usage) + "%")
        cmd = "cat /sys/class/thermal/thermal_zone0/temp"
        result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
        try:
            temperature = int(float(result.stdout.decode('utf-8').replace('\n', '')) / 1000)
            text = cpu_usage + " " + str(temperature) + ""
        except ValueError:
            text = cpu_usage
            pass

        txt = "^s{}^{} {} {}{} ".format(name, icon_color, icon, text_color, text)
        common.write_to_file(txt + "\n", str(name))
        if not loop:
            if set_root:
                os.system("xsetroot -name '" + str(txt) + "'")
            break
        time.sleep(DELAY_TIME)


def update_thread():
    _thread.start_new_thread(update, (False, False))


def notify():
    cmd = 'notify-send "{title}" "{message}" -r {id}'.format(
        title='  CPU tops',
        message='$(ps axch -o cmd:15,%cpu --sort=-%cpu | head  | sed \'s/$/&\\%\\n/g\')',
        id=1014
    )
    os.system(cmd)


def click(string=''):
    match string:
        case 'L':
            notify()
        case 'M':
            pass
        case 'R':
            os.system("st -t statusutil -c floatingTerminal -g 84x26 -e btop")
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
