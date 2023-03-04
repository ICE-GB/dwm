#! /usr/bin/env bash

temp_file=$(
  cd "$(dirname "$0")" || exit
  cd ..
  pwd
)/temp

this=_music
icon_color="^c#3E206F^^b#6E51760x88^"
text_color="^c#3E206F^^b#6E51760x99^"
signal=$(echo "^s$this^" | sed 's/_//')

icon=""
status=""
song=""
escape_result=""

music_title_show_length=""

# check
[ ! "$(command -v mpc)" ] && echo command not found: mpc && exit

save() {
  sed -i '/^export '"$1"'=.*$/d' "$temp_file"
  printf "export %s='%s'\n" "$1" "$2" >>"$temp_file"
}

escape() {
  # todo 完善转义
  param="$1"
  escape_result=""
  for i in $(seq ${#param}); do
    escape_tmp_i=${param:$i-1:1}
    if [ "'" == "$escape_tmp_i" ]; then
      escape_tmp_i="’"
    fi
    escape_result="$escape_result$escape_tmp_i"
  done
  #  notify-send -r 9527 -h string:hlcolor:#dddddd "$escape_result"
}

update() {
  song=$(mpc current)
  sed -i '/^export '$this'=.*$/d' "$temp_file"
  if [ "" != "$song" ]; then
    escape "$song"
    song="$escape_result"

    # shellcheck source=../temp
    source "$temp_file" # 从 temp 文件中读取模块的状态
    if [ "" == "$music_title_show_i" ]; then
      music_title_show_i=0
    fi

    bar_length=20
    music_title_show_length=$((${#song} - bar_length))

    song=${song:$music_title_show_i:$bar_length}

    if [ $(mpc status | grep -c playing) -gt 0 ]; then
      music_title_show_i=$((music_title_show_i + 1))
    fi

    if [[ $music_title_show_i -gt $music_title_show_length ]]; then
      music_title_show_i=0
    fi

    save "music_title_show_i" $music_title_show_i

    printf "export %s=\"%s%s%s%s%s\"\n" $this "$signal" "$icon_color" " $icon " "$text_color" " $song " >>"$temp_file"
  else
    printf "export %s=\"%s%s%s\"\n" $this "$signal" "$icon_color" " $icon  " >>"$temp_file"
  fi
}

notify() {
  update
  song=$(mpc current)
  notify-send -r 9527 -h string:hlcolor:#dddddd "$icon $status $song"
}

play_or_pause() {
  if [[ $(mpc pause-if-playing) ]]; then
    status="paused"
  else
    mpc -q play
    status="playing"
  fi
  song=$(mpc current)
}

click() {
  case "$1" in
  L)
    play_or_pause
    notify
    ;; # 播放或暂停
  M)
    mpc -q stop
    status="stop"
    notify
    ;; # 停止
  R)
    st -t ncmpcpp -c FGN -e ncmpcpp
    ;; # 打开 ncmpcpp
  D)
    mpc -q next
    status="playing"
    notify
    ;; # 下一首
  U)
    mpc -q prev
    status="playing"
    notify
    ;; # 上一首
  esac
}

case "$1" in
click) click "$2" ;;
notify) notify ;;
*) update ;;
esac
