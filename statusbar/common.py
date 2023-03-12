#!/usr/bin/env python

import os
# import sys
# import subprocess
import re
import threading

# import time

PACKAGES_LISTS = {
    'music': 1,
    'wifi': 2,
    'net': 1,
    'cpu': 2,
    'memory': 2,
    'vol': 1,
    'battery': 3,
    'date': 1,
    'icon': 100,
}

DWM_PATH = "/home/gb/.dwm/"
PACKAGES_PATH = DWM_PATH + "statusbar/"
TEMP_FILE = "/home/gb/python_tmp"

MUSIC_PROGRAM = "yesplaymusic"

black = "#1e222a"
white = "#D8DEE9"
grey = "#373d49"
blue = "#81A1C1"
blue2 = "#5E81AC"
blue3 = "#88C0D0"
blue4 = "#8FBCBB"
red = "#d47d85"
green = "#A3BE8C"
pink = "#B48EAD"
yellow = "#EBCB8B"
orange = "#D08770"
darkblue = "#7292b2"

threadLock = threading.Lock()


def write_to_file(string, package_name):
    threadLock.acquire()
    if not os.path.exists(TEMP_FILE):
        os.system("touch " + TEMP_FILE)
    with open(TEMP_FILE, 'r+') as f:
        lines = f.readlines()
    with open(TEMP_FILE, 'w+') as f:
        find = False
        for line in lines:
            if re.match("^\\^s", line) is None:
                continue
            flag = re.match("^\\^s" + package_name, line)
            if flag is None:
                f.write(line)
            else:
                f.write(string)
                find = True
        if not find:
            f.write(string)
    threadLock.release()


if __name__ == "__main__":
    pass
