#! /usr/bin/env bash

# source "$HOME"/.profile
# source "$HOME"/.dwm/.profile

while true; do
  log_file=$DWM/logs/picom-$(date -d "today" +"%Y%m%d%H%M").log

  picom_need_experimental=$(picom --help | grep -c experimental-backends) # 开启picom
  if [ "$picom_need_experimental" -ge 1 ]; then
    picom --experimental-backends --config "$DWM"/.config/picom.conf >>"$log_file" 2>&1
  else
    picom --config "$DWM"/.config/picom.conf >>"$log_file" 2>&1
  fi
done
