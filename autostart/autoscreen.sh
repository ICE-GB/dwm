#! /usr/bin/env bash

# autorandr --change

if [ "$(uname -n)" == "kubuntu2210" ]; then
  xrandr --dpi 120
  xrandr --output Virtual1 --mode 2560x1440
elif [ "$(uname -n)" == "gb-HP-ProBook-455R-G6" ]; then
  INNER_PORT=$(xrandr | grep -w 'connected' | head -1 | grep -w 'connected' | awk '{print $1}')
  # 查找已连接的外接接口
  OUT_PORT_CONNECTED=$(xrandr | grep -v "$INNER_PORT" | grep -w 'connected' | awk '{print $1}')
  if [ "" = "$OUT_PORT_CONNECTED" ]; then
    xrandr --output "$INNER_PORT" --mode 1920x1080 --pos 0x0 --scale 1x1 --primary
  else
    xrandr --output "$INNER_PORT" --off --output "$OUT_PORT_CONNECTED" --mode 1920x1080 --pos 0x0 --scale 1x1 --primary
  fi
elif [ "$(uname -n)" == "nixos-awnlzw" ]; then
  xrandr --dpi 120
  xrandr --output DP-4 --mode 2560x1600 --pos 0x0 --primary
  # xrandr --output HDMI-0 --mode 2560x1440 --pos 2560x0 --primary
fi
