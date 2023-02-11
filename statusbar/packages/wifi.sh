#! /bin/bash

tempfile=$(
  cd $(dirname $0)
  cd ..
  pwd
)/temp

this=_wifi
icon_color="^c#000080^^b#3870560x88^"
text_color="^c#000080^^b#3870560x99^"
signal=$(echo "^s$this^" | sed 's/_//')

# check
[ ! "$(command -v nmcli)" ] && echo command not found: nmcli && exit

# 中英文适配
wifi_grep_keyword="connected to"
wifi_disconnected="disconnected"
wifi_disconnected_notify="disconnected"

wifi_grep_keyword_zh_CN="已连接 到"
wifi_disconnected_zh_CN="未连接"
wifi_disconnected_notify_zh_CN="未连接到网络"


update() {
  wifi_icon="褐"
  wifi_text=$(nmcli | grep "$wifi_grep_keyword" | sed "s/$wifi_grep_keyword//" | awk '{print $2}' | paste -d " " -s)
  [ "$wifi_text" = "" ] && wifi_text=$(nmcli | grep "$wifi_grep_keyword_zh_CN" | sed "s/$wifi_grep_keyword_zh_CN//" | awk '{print $2}' | paste -d " " -s)
  [ "$wifi_text" = "" ] && wifi_text=$wifi_disconnected

  icon=" $wifi_icon "
  text=" $wifi_text "

  sed -i '/^export '$this'=.*$/d' $tempfile
  printf "export %s='%s%s%s%s%s'\n" $this "$signal" "$icon_color" "$icon" "$text_color" "$text" >>$tempfile
}

notify() {
  update
  notify-send -r 9527 "$wifi_icon Network" "\n$wifi_text"
}

call_nm() {
  kcmshell5 kcm_networkmanagement
}

click() {
  case "$1" in
  L) notify ;;
  R) call_nm ;;
  esac
}

case "$1" in
click) click $2 ;;
notify) notify ;;
*) update ;;
esac
