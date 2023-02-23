#! /usr/bin/env bash
# VOL 音量脚本
# 本脚本需要你自行修改音量获取命令
# 例如我使用的是 pipewire
#
# $ pactl lisk sinks | grep RUNNING -A 8
#         State: RUNNING
#         Name: bluez_output.88_C9_E8_14_2A_72.1
#         Description: WH-1000XM4
#         Driver: PipeWire
#         Sample Specification: float32le 2ch 48000Hz
#         Channel Map: front-left,front-right
#         Owner Module: 4294967295
# 静音 -> Mute: no
# 音量 -> Volume: front-left: 13183 /  20% / -41.79 dB,   front-right: 13183 /  20% / -41.79 dB

temp_file=$(
  cd "$(dirname "$0")" || exit
  cd ..
  pwd
)/temp

this=_vol
icon_color="^c#442266^^b#7879560x88^"
text_color="^c#442266^^b#7879560x99^"
signal=$(echo "^s$this^" | sed 's/_//')

# check
[ ! "$(command -v pactl)" ] && echo command not found: pactl && exit

update() {
  sink=$(pactl info | grep 'Default Sink' | awk '{print $3}')
  [ "$sink" = "" ] && sink=$(pactl info | grep '默认音频入口' | awk '{print $2}')
  muted=$(pactl get-default-sink | xargs pactl get-sink-mute | grep 'Mute: 否')
  [ "$muted" = "" ] && muted=$(pactl get-default-sink | xargs pactl get-sink-mute | grep 'Mute: no')
  vol_text=$(pactl get-default-sink | xargs pactl get-sink-volume | head -1 | awk '{print $5}')
  vol_int=$(echo "$vol_text" | grep -Eo "[0-9]+")
  if [ ! "$muted" ]; then
    vol_text="mute"
    vol_icon="ﱝ"
  elif [ "$vol_int" -eq 0 ]; then
    vol_text="00"
    vol_icon="婢"
  elif [ "$vol_int" -lt 10 ]; then
    vol_icon="奔"
    vol_text=0$vol_text
  elif [ "$vol_int" -le 50 ]; then
    vol_icon="奔"
  else vol_icon="墳"; fi

  icon=" $vol_icon "
  text=" $vol_text "

  sed -i '/^export '$this'=.*$/d' "$temp_file"
  printf "export %s='%s%s%s%s%s'\n" $this "$signal" "$icon_color" "$icon" "$text_color" "$text" >>"$temp_file"
}

notify() {
  update
  notify-send -r 9527 -h int:value:$vol_text -h string:hlcolor:#dddddd "$vol_icon Volume"
}

click() {
  case "$1" in
  L) notify ;;                                    # 仅通知
  M) pactl set-sink-mute @DEFAULT_SINK@ toggle ;; # 切换静音
  R) pavucontrol ;;                               # 配置音频设备和音量
  U)
    pactl set-sink-volume @DEFAULT_SINK@ +5%
    notify
    ;; # 音量加
  D)
    pactl set-sink-volume @DEFAULT_SINK@ -5%
    notify
    ;; # 音量减
  esac
}

case "$1" in
click) click "$2" ;;
notify) notify ;;
*) update ;;
esac
