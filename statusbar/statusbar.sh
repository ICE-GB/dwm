#! /usr/bin/env bash

_this_dir=$(
  cd "$(dirname "$0")" || exit
  pwd
)
temp_file="$_this_dir"/temp
touch "$temp_file"

# 设置某个模块的状态 update cpu mem ...
update() {
  [ ! "$1" ] && refresh && return    # 当指定模块为空时 结束
  bash "$_this_dir"/packages/"$1".sh # 执行指定模块脚本
  shift 1
  update "$@" # 递归调用
}

# 处理状态栏点击
click() {
  [ ! "$1" ] && return                          # 未传递参数时 结束
  bash "$_this_dir"/packages/"$1".sh click "$2" # 执行指定模块脚本
  update "$1"                                   # 更新指定模块
  refresh                                       # 刷新状态栏
}

# 更新状态栏
refresh() {
  _music=''
  _net_speed=''
  _icons=''
  _cpu=''
  _mem=''
  _date=''
  _vol=''
  _bat='' # 重置所有模块的状态为空
  # shellcheck source=./temp
  source "$temp_file"                                                  # 从 temp 文件中读取模块的状态
  xsetroot -name "$_net_speed$_cpu$_mem$_music$_vol$_date$_bat$_icons" # 更新状态栏
}

# 启动定时更新状态栏 不同的模块有不同的刷新周期 注意不要重复启动该func
cron() {
  echo >"$temp_file"
  ((i = 0))
  while true; do
    to=()                                                                     # 存放本次需要更新的模块
    [ $((i % 10)) -eq 0 ] && to=("${to[@]}" wifi)                             # 每 10秒  更新 wifi
    [ $((i % 1)) -eq 0 ] && to=("${to[@]}" cpu mem vol icons music net_speed) # 每 1秒  更新 cpu mem vol icons net_speed music
    [ $((i % 300)) -eq 0 ] && to=("${to[@]}" bat)                             # 每 300秒 更新 bat
    [ $((i % 1)) -eq 0 ] && to=("${to[@]}" date)                              # 每 1秒   更新 date
    [ "$i" -lt 30 ] && to=(wifi cpu mem date vol icons bat music net_speed)   # 前 30秒  更新所有模块
    update "${to[@]}"                                                         # 将需要更新的模块传递给 update
    sleep 1
    ((i += 1))
  done &
}

# cron 启动定时更新状态栏
# update 更新指定模块 `update cpu` `update mem` `update date` `update vol` `update bat` 等
# updateall 更新所有模块 | check 检查模块是否正常(行为等于updateall)
# * 处理状态栏点击 `cpu 按键` `mem 按键` `date 按键` `vol 按键` `bat 按键` 等
case $1 in
cron) cron ;;
update)
  shift 1
  update "$*"
  ;;
updateall | check) update icons wifi cpu mem date vol bat ;;
*) click "$1" "$2" ;; # 接收click statusbar传递过来的信号 $1: 模块名  $2: 按键(L|M|R|U|D)
esac
