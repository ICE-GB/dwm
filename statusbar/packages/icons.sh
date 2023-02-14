#! /bin/bash
# ICONS 部分特殊的标记图标 这里是我自己用的，你用不上的话去掉就行

temp_file=$(
  cd "$(dirname "$0")" || exit
  cd ..
  pwd
)/temp

this=_icons
color="^c#2D1B46^^b#5555660x66^"
signal=$(echo "^s$this^" | sed 's/_//')

with_v2raya() {
  [ "$(pgrep -f 'v2raya')" ] && icons=("${icons[@]}" "")
}

with_bluetooth() {
  # 此处为自用蓝牙设备的 MAC 地址，你可以自定义该部分
  [ ! "$(command -v bluetoothctl)" ] && echo command not found: bluetoothctl && return
  (bluetoothctl info F4:0E:11:82:77:CC | grep -q 'Connected: yes') && icons=("${icons[@]}" "")
}

update() {
  icons=("")
  with_v2raya
  with_bluetooth

  text=" ${icons[*]} "

  sed -i '/^export '$this'=.*$/d' "$temp_file"
  printf "export %s='%s%s%s'\n" $this "$signal" "$color" "$text" >>"$temp_file"
}

notify() {
  texts=""
  [ "$(pgrep -f 'v2raya')" ] && texts="$texts\n v2raya 已启动"
  (bluetoothctl info F4:0E:11:82:77:CC | grep -q 'Connected: yes') && texts="$texts\n Cleer ARC 已连接"
  [ "$texts" != "" ] && notify-send " Info" "$texts" -r 9527
}

call_menu() {
  ~/.config/rofi/scripts/powermenu_t2
}

click() {
  case "$1" in
  L)
    notify
    feh --randomize --bg-fill ~/Pictures/wallpaper/*.*
    ;;
  R) call_menu ;;
  esac
}

case "$1" in
click) click "$2" ;;
notify) notify ;;
*) update ;;
esac
