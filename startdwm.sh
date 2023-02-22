#! /usr/bin/env bash

source "$HOME"/.profile
source "$HOME"/.dwm/.profile

"$HOME"/scripts/set_screen.sh two # 设置显示器

# 在不登出和退出程序程序的情况下重启dwm
# super ctrl f12 重启
# pkill dwm 真正地退出
# https://wiki.archlinuxcn.org/wiki/Dwm#:~:text=%E5%9C%A8%E4%B8%8D%E7%99%BB%E5%87%BA%E5%92%8C%E9%80%80%E5%87%BA%E7%A8%8B%E5%BA%8F%E7%A8%8B%E5%BA%8F%E7%9A%84%E6%83%85%E5%86%B5%E4%B8%8B%E9%87%8D%E5%90%AFdwm
while true; do
  log_file=$DWM/logs/dwm-$(date -d "today" +"%Y%m%d%H%M").log
  # Log stderror to a file
  # dwm 2> $DWM/dwm.log
  # No error logging
  #dwm >/dev/null 2>&1
  # log all
  # dwm >>$DWM/dwm.log 2>>$DWM/dwm.log
  # log stderror to a file
  dwm 2>"$log_file"
done
