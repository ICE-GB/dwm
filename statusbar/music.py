#!/usr/bin/env python

import _thread
import os
import re
import subprocess
import sys
import time

import common

music_program = common.MUSIC_PROGRAM

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


def get_music_title():
    cmd = "mpc current"
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    title = result.stdout.decode('utf-8').replace('\n', '')
    title = title.replace("'", "")  # 解决一些歌曲带'的问题
    return title


def is_playing():
    cmd = "mpc status | grep -c playing"
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    result = result.stdout.decode('utf-8').replace('\n', '')
    if int(result) > 0:
        return True
    else:
        return False


def save_cache(var, value):
    music_cache = create_cache(var)
    cmd = "echo {} > {} ".format(value, music_cache)
    subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)


def get_cache(var):
    music_cache = create_cache(var)
    cmd = "cat " + music_cache
    result = subprocess.run(cmd, shell=True, timeout=3, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    value = result.stdout.decode('utf-8').replace("\n", "")
    if value == "":
        value = 0
    return value


def create_cache(var):
    cache_dir = "~/.cache/"
    music_cache = cache_dir + var
    if not os.path.exists(music_cache):
        os.system("touch " + music_cache)
    return music_cache


def update(loop=False, set_root=True):
    while True:
        icon = "ﱘ"
        text = get_music_title()
        if "" == text:
            txt = "^s{}^{} {} ".format(name, icon_color, icon)
        else:
            bar_length = 20
            music_title_pos = get_cache("music_title_pos")
            music_title_pos = int(music_title_pos)
            music_title_show_length = len(text) - bar_length
            text = text[music_title_pos:music_title_pos + bar_length]
            if is_playing():
                music_title_pos = music_title_pos + 1
            if music_title_pos > music_title_show_length:
                music_title_pos = 0
            save_cache("music_title_pos", music_title_pos)
            txt = "^s{}^{} {} {} {} ".format(name, icon_color, icon, text_color, text)
        common.write_to_file(txt + "\n", str(name))
        if not loop:
            if set_root:
                print(txt)
                # os.system("xsetroot -name '"+str(txt)+"'")
            break
        time.sleep(DELAY_TIME)


def update_thread():
    _thread.start_new_thread(update, (False, False))


def play_or_pause():
    result = os.system("mpc pause-if-playing")
    if 0 == result:
        status = "paused"
    else:
        os.system("mpc -q play")
        status = "playing"
    return status


def click(string=''):
    match string:
        case 'L':
            play_or_pause()
        case 'M':
            os.system("mpc -q stop")
            pass
        case 'R':
            os.system("xdotool keydown Super m keyup m Super")
            pass
        case 'U':
            os.system("mpc -q prev")
            pass
        case 'D':
            os.system("mpc -q next")
            pass
        case _:
            pass


def notify():
    pass


if __name__ == "__main__":
    print(sys.argv)
    if len(sys.argv) > 1:
        if sys.argv[1] == "update":
            pass
        else:
            click(sys.argv[1])
            update(set_root=False)
    else:
        update()
