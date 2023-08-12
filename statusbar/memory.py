#!/usr/bin/env python

import _thread
import os
import re
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
        icon = "󰍛"
        mem = int(psutil.virtual_memory()[2])
        if mem > 80:
            kill_some_thing()
        if mem > 90:
            notify()
        text = str(mem) + "%"
        txt = "^s{}^{} {} {}{} ".format(name, icon_color, icon, text_color, text)
        common.write_to_file(txt + "\n", str(name))
        if not loop:
            if set_root:
                os.system("xsetroot -name '" + str(txt) + "'")
            break
        time.sleep(DELAY_TIME)


def kill_some_thing():
    cmd = 'pkill -f barrier && barrier &'
    os.system(cmd)


def update_thread():
    _thread.start_new_thread(update, (False, False))


def click(string=''):
    match string:
        case 'L':
            notify()
            pass
        case 'M':
            pass
        case 'R':
            pass
            os.system("st -t statusutil -c floatingTerminal -g 84x26 -e btop")
        case 'U':
            pass
        case 'D':
            pass
        case _:
            pass


def notify():
    cmd = 'notify-send "{title}" "{message}" -r {id}'.format(
        title='󰍛  MEM tops',
        message='$(ps axch -o cmd:15,%mem --sort=-%mem | head  | sed \'s/$/&\\%\\n/g\')',
        id=1015
    )
    os.system(cmd)


if __name__ == "__main__":
    if len(sys.argv) > 1:
        if sys.argv[1] == "update":
            pass
        else:
            click(sys.argv[1])
            update(set_root=False)
    else:
        update()
