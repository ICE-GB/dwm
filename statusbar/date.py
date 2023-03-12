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
DELAY_TIME = 1

filename = os.path.basename(__file__)
name = re.sub("\\..*", '', filename)


def update(loop=False, set_root=True):
    while True:
        icon = ""
        text = time.strftime(" %H:%M:%S ", time.localtime())
        txt = "^s" + str(name) + "^" + str(icon_color) + str(icon) + str(text_color) + str(text)
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
        title=' Calendar',
        message='\nData: $(date \'+%y-%m-%d \\nTime: %T\')',
        id=9540
    )
    os.system(cmd)


def click(string=''):
    match string:
        case 'L':
            notify()
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
