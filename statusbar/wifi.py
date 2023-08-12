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
DELAY_TIME = 2

filename = os.path.basename(__file__)
name = re.sub("\\..*", '', filename)
icon_map = {
    "wifi": "",
    "ethernet": "󰈀",
    "offline": ""
}

cmd_map = {
    "wifi": "iwconfig 2>/dev/null | awk '/ESSID:/ {print $4}' | sed 's/ESSID:\"//g;s/\"$//g'",
    "ethernet": "ip route get 1.1.1.1 | awk '{print $5}'",
    "offline": "echo 'Unknown'"
}


def get_network_status():
    try:
        # 使用 subprocess.run() 启动 nmcli 命令获取网络状态
        cmd = "nmcli -t -f TYPE,STATE -e no connection show --active"
        result = subprocess.run(cmd, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        out = result.stdout.decode().strip()

        # 解析 nmcli 命令得到的结果，判断是否为已连接状态
        if out:
            if ("wifi" in out or "wireless" in out) and ("connected" in out or "activated" in out):
                return "wifi"
            elif "ethernet" in out and ("connected" in out or "activated" in out):
                return "ethernet"
    except subprocess.CalledProcessError:
        # 处理 nmcli 命令无法执行的情况
        pass

    return "offline"


def update(loop=False, set_root=True):
    while True:
        icon = icon_map[get_network_status()]
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
    connect_status = get_network_status()
    net_name = subprocess.check_output(cmd_map[connect_status], shell=True).decode("utf-8").strip()
    cmd = "notify-send '{} 已连接到 {}'".format(icon_map[connect_status], net_name)
    os.system(cmd)


def click(string=''):
    match string:
        case 'L':
            notify()
        case 'M':
            os.system("nm-connection-editor")
            pass
        case 'R':
            os.system("kitty -T nmtui --class floatingTerminal -e nmtui ")
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
