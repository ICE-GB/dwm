#!/usr/bin/env python

import _thread
import os
import re
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
DELAY_TIME = 10

filename = os.path.basename(__file__)
name = re.sub("\\..*", '', filename)


def update(loop=False, set_root=True):
    while True:
        icon = ""
        text = ""
        txt = "^s{}^{} {} {}{} ".format(name, icon_color, icon, text_color, text)
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
            os.system("~/.config/rofi/scripts/powermenu_t2")
            pass
        case 'M':
            pass
        case 'R':
            pass
            os.system("feh --randomize --bg-fill ~/Pictures/wallpaper/*.*")
        case 'U':
            pass
        case 'D':
            pass
        case _:
            pass


def notify():
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
