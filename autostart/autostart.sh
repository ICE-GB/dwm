#! /usr/bin/env bash

# source "$HOME"/.profile
source "$HOME"/.dwm/.profile

# If you find fcitx5 icon is located at the most left of the straybar, please increase the delay value
sleep 1 # need to wait dwm start complete and fcitx5 start complete

settings() {
  [ "$1" ] && sleep "$1"
  feh --randomize --bg-fill ~/Pictures/wallpaper/*.* # 设置壁纸
  xset -b                            # 关闭蜂鸣器
  syndaemon -i 1 -t -K -R -d         # 设置使用键盘时触控板短暂失效
  xrdb -merge ~/.dwm/xresources      # 为st进行设置
}

daemons() {
  [ "$1" ] && sleep "$1"

  pkill -f statusbar.sh
  pkill -f statusbar.py
  pkill fcitx5
  pkill flameshot
  pkill dunst
  pkill barrier
  pkill -f start_picom.sh
  pkill picom
  pkill pcmanfm
  pkill greenclip

  [ "$1" ] && sleep "$1"

  if [ 0 == "$(pgrep -c picom)" ]; then
    ~/.dwm/.bin/start_picom.sh & # 开启picom
  fi

  command -v nix-shell >/dev/null 2>&1

  if [ $? -eq 0 ]; then
    echo "nix-shell command exists"
    nix-shell ~/.dwm/shell.nix --run "python3 ~/.dwm/statusbar/statusbar.py cron >/dev/null &"
  else
    echo "nix-shell command does not exist"
    python3 ~/.dwm/statusbar/statusbar.py cron >/dev/null &
  fi

  fcitx5 &                                                 # 开启输入法
  flameshot &                                              # 截图要跑一个程序在后台 不然无法将截图保存到剪贴板
  dunst &                                                  # 开启通知server
  pcmanfm -d &                                             # 开启PCManFM
  greenclip daemon >>~/.dwm/logs/greenclip.log 2>&1 & # 开启剪切板


  # cfw & # clash for windows
  # crow & # translate
  # blueman-manager & # bluetooth manager
  # copyq & # copy software

  # libinput-gestures-setup start # touchpad open gesture
  # xinput --set-prop 15 'libinput Accel Speed' 0.5 # set touchpad sensitivity

  #  lemonade server &                                                                     # 开启lemonade 远程剪切板支持
  #  xss-lock -- ~/scripts/blur_lock.sh &                                                   # 开启自动锁屏程序

  if [[ ! "$(command -v barrier)" ]]; then
    echo "command not found: barrier"
  else
    barrier & # 键鼠共享
  fi

  if [[ ! "$(command -v /usr/bin/vmware-user-suid-wrapper)" ]]; then
    echo "command not found: /usr/bin/vmware-user-suid-wrapper"
  else
    /usr/bin/vmware-user-suid-wrapper & # 开启open-vm-tools-desktop
  fi
}

# Notice that cron need exec before other program
cron() {
  [ $1 ] && sleep $1
  let i=10
  while true; do
      [ $((i % 10)) -eq 0 ] && ~/.dwm/autostart/autoscreen.sh # check screen and autoset
      # [ $((i % 300)) -eq 0 ] && feh --randomize --bg-fill ~/Pictures/wallpaper/*.*
      sleep 5; let i+=5
  done
}

settings 1 & # 初始化设置项
cron 5 &     # 定时任务项
daemons 1 &  # 后台程序项

# xhost + # add support for docker gui app
