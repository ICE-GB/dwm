#! /usr/bin/env bash

source "$HOME"/.profile
source "$HOME"/.dwm/.profile

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
  echo "由lightdm处理"
fi


# "$HOME"/scripts/set_screen.sh two # 设置显示器

# 在不登出和退出程序程序的情况下重启dwm
# super ctrl f12 重启
# pkill dwm 真正地退出
# https://wiki.archlinuxcn.org/wiki/Dwm#:~:text=%E5%9C%A8%E4%B8%8D%E7%99%BB%E5%87%BA%E5%92%8C%E9%80%80%E5%87%BA%E7%A8%8B%E5%BA%8F%E7%A8%8B%E5%BA%8F%E7%9A%84%E6%83%85%E5%86%B5%E4%B8%8B%E9%87%8D%E5%90%AFdwm
while true; do
  log_file=$DWM/logs/dwm-$(date -d "today" +"%Y%m%d%H%M")-$(date +%N).log
  # Log stderror to a file
  # dwm 2> $DWM/dwm.log
  # No error logging
  #dwm >/dev/null 2>&1
  # log all
  # dwm >>$DWM/dwm.log 2>>$DWM/dwm.log
  # log stderror to a file
  # dwm 2>"$log_file"

  run_dwm="$HOME"/.dwm/dwm
  if [ -f "$run_dwm" ]; then
    echo "$run_dwm"
    $run_dwm 2>"$log_file"
  else
    dwm 2>"$log_file"
  fi
  sleep 3s
done
