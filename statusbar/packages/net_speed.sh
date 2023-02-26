#! /usr/bin/env bash

temp_file=$(
  cd "$(dirname "$0")" || exit
  cd ..
  pwd
)/temp

this=_net_speed
text_color="^c#000080^^b#3870560x88^"
signal=$(echo "^s$this^" | sed 's/_//')

TMP=""
net_speed_text=""

save() {
  sed -i '/^export '"$1"'=.*$/d' "$temp_file"
  printf "export %s='%s'\n" "$1" "$2" >>"$temp_file"
}

calculate() {
  TMP=$(printf "%.1f" "$(echo "scale=1;$1/$2" | bc)")"$3"
}

human_read() {
  if [[ $1 -lt 1024 ]]; then
    calculate "$1" 1 "B/s"
  elif [[ $1 -gt 1048576 ]]; then
    calculate "$1" 1048576 "MB/s"
  else
    calculate "$1" 1024 "KB/s"
  fi
}

net_speed() {
  interface="$1"
  if [ "" == "$interface" ]; then
    interface="ens33"
  fi

  # shellcheck source=../temp
  source "$temp_file" # 从 temp 文件中读取模块的状态

  if [ "" == "$RX_pre" ]; then
    RX_pre=$(grep "$interface" /proc/net/dev | sed 's/:/ /g' | awk '{print $2}')
    TX_pre=$(grep "$interface" /proc/net/dev | sed 's/:/ /g' | awk '{print $10}')
  fi

  RX_next=$(grep "$interface" /proc/net/dev | sed 's/:/ /g' | awk '{print $2}')
  TX_next=$(grep "$interface" /proc/net/dev | sed 's/:/ /g' | awk '{print $10}')

  RX=$((RX_next - RX_pre))
  TX=$((TX_next - TX_pre))

  human_read $RX

  RX=$TMP

  human_read $TX

  TX=$TMP

  net_speed_text=" $RX  $TX"

  RX_pre=$RX_next
  TX_pre=$TX_next

  save RX_pre "$RX_next"
  save TX_pre "$TX_next"
}

update() {
  net_speed "ens33"
  sed -i '/^export '$this'=.*$/d' "$temp_file"
  printf "export %s='%s%s%s'\n" $this "$signal" "$text_color" " $net_speed_text " >>"$temp_file"
}

notify() {
  update
  notify-send -r 9527 "褐 Network" "\n$net_speed_text"
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
click) click "$2" ;;
notify) notify ;;
*) update ;;
esac
