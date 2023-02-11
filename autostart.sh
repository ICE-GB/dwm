#! /bin/bash
# DWM自启动脚本 仅作参考
# 搭配 https://github.com/yaocccc/scripts 仓库使用 目录位置 ~/scripts
# 部分配置文件在 ~/scripts/config 目录下

source ~/.profile

settings() {
  [ $1 ] && sleep $1
  xset -b                     # 关闭蜂鸣器
  syndaemon -i 1 -t -K -R -d  # 设置使用键盘时触控板短暂失效
}

daemons() {
  [ $1 ] && sleep $1

  pkill statusbar.sh
  pkill fcitx5
  pkill flameshot
  pkill dunst
  pkill barrier
  pkill picom

  [ $1 ] && sleep $1

  $DWM/statusbar/statusbar.sh cron &                                           # 开启状态栏定时更新
  fcitx5 &                                                                     # 开启输入法
  flameshot &                                                                  # 截图要跑一个程序在后台 不然无法将截图保存到剪贴板
  barrier &                                                                    # 键鼠共享
  dunst -conf ~/scripts/config/dunst.conf &                                    # 开启通知server
  picom_need_experimental=$(picom --help | grep experimental-backends | wc -l) # 开启picom
  picom_need_experimental=0
  if [ "$picom_need_experimental" -ge 1 ]; then
    picom --experimental-backends --config ~/scripts/config/picom.conf >>/dev/null 2>&1 &                  
  else
    picom --config ~/scripts/config/picom.conf >>/dev/null 2>&1 &
  fi

  #  lemonade server &                                                                     # 开启lemonade 远程剪切板支持
  #  xss-lock -- ~/scripts/blurlock.sh &                                                   # 开启自动锁屏程序

  [ ! "$(command -v /usr/bin/vmware-user-suid-wrapper)" ] && echo command not found: /usr/bin/vmware-user-suid-wrapper || /usr/bin/vmware-user-suid-wrapper & # 开启open-vm-tools-desktop
}

cron() {
  feh --randomize --bg-fill ~/Pictures/wallpaper/*.*
  #  [ $1 ] && sleep $1
  #  let i=10
  #  while true; do
  #    #        [ $((i % 10)) -eq 0 ] && ~/scripts/set_screen.sh check # 每10秒检查显示器状态 以此自动设置显示器
  #    [ $((i % 300)) -eq 0 ] && feh --randomize --bg-fill ~/Pictures/wallpaper/*.* # 每300秒更新壁纸
  #    sleep 10
  #    let i+=10
  #  done
}

settings 1 & # 初始化设置项
daemons 3 &  # 后台程序项
cron 5 &     # 定时任务项
