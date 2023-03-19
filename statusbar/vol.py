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
DELAY_TIME = 1

filename = os.path.basename(__file__)
name = re.sub("\\..*", '', filename)

vol_text = "--"
vol_icon = "ﱝ"
volumuted = ""


def get_vol_content():
    global vol_text
    global vol_icon
    global volumuted

    cmd = "echo $(LANG=en.US.UTF-8 pactl info | grep 'Default Sink' | awk '{print $3}')"
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    sink = str(result.stdout.decode('utf-8').replace('\n', ''))

    cmd = "echo $(LANG=en.US.UTF-8 pactl list sinks | grep " + str(sink) + " -A 6 | sed -n '7p' | grep 'Mute: no')"
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    volumuted = str(result.stdout.decode('utf-8').replace('\n', ''))

    cmd = "echo $(LANG=en.US.UTF-8 pactl list sinks | grep " + str(sink) + " -A 7 | sed -n '8p' | awk '{printf int($5)}' )"
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    vol_text = str(result.stdout.decode('utf-8').replace('\n', ''))

    if volumuted == "":
        vol_text = "--"
        vol_icon = "ﱝ"
    else:
        vol = int(vol_text)
        vol_text = vol_text
        if vol == 0:
            vol_icon = "婢"
            vol_text = "00"
        # elif vol<10 : vol_icon="奔"
        # elif vol<50 : vol_icon="奔"
        else:
            vol_icon = "墳"
    vol_full = "{} {}%".format(vol_icon, vol_text)
    return vol_full
    # return str(vol_icon)+str(vol_text)+"%"+" "+GetBluetoothBatteryByPactl()
    # return str(vol_icon)+str(vol_text)+"%"+" "+GetBluetoothBattery()
    # return str(vol_icon)+str(vol_text)+"%"


def update(loop=False, set_root=True):
    while True:
        icon = get_vol_content()
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
    global vol_text
    global vol_icon
    global volumuted
    get_vol_content()

    if volumuted == "":
        cmd = "notify-send -r 9527 '婢  mute'  "
    else:
        cmd = "notify-send -r 9527 -h int:value:" + str(int(vol_text)) + " -h string:hlcolor:#dddddd " + '"' + str(
            vol_icon) + " Volume" + '"'
    os.system(cmd)
    pass


def click(string=''):
    match string:
        case 'L':
            notify()
            pass
        case 'M':
            os.system("pactl set-sink-mute @DEFAULT_SINK@ toggle")
            notify()
            pass
        case 'R':
            os.system("killall pavucontrol || pavucontrol --class floatingTerminal &")
            pass
        case 'U':
            pass
            os.system("pactl set-sink-volume @DEFAULT_SINK@ +5%; notify")
            notify()
        case 'D':
            os.system("pactl set-sink-volume @DEFAULT_SINK@ -5%; notify")
            notify()
            pass
        case _:
            pass


if __name__ == "__main__":
    if len(sys.argv) > 1:
        if sys.argv[1] == "update":
            pass
        elif sys.argv[1] == "notify":
            notify()
        else:
            update(set_root=False)
            click(sys.argv[1])
            update(set_root=False)
    else:
        update()
