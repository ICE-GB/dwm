#! /bin/bash
# DATE 获取日期和时间的脚本

temp_file=$(
  cd "$(dirname "$0")" || exit
  cd ..
  pwd
)/temp

this=_date
icon_color="^c#4B005B^^b#7E51680x88^"
text_color="^c#4B005B^^b#7E51680x99^"
signal=$(echo "^s$this^" | sed 's/_//')

update() {
  time_text="$(date '+%m/%d %H:%M')"
  case "$(date '+%I')" in
  "01") time_icon="" ;;
  "02") time_icon="" ;;
  "03") time_icon="" ;;
  "04") time_icon="" ;;
  "05") time_icon="" ;;
  "06") time_icon="" ;;
  "07") time_icon="" ;;
  "08") time_icon="" ;;
  "09") time_icon="" ;;
  "10") time_icon="" ;;
  "11") time_icon="" ;;
  "12") time_icon="" ;;
  esac

  icon=" $time_icon "
  text=" $time_text "

  sed -i '/^export '$this'=.*$/d' "$temp_file"
  printf "export %s='%s%s%s%s%s'\n" $this "$signal" "$icon_color" "$icon" "$text_color" "$text" >>"$temp_file"
}

notify() {
  _cal=$(cal | sed 's/..7m/<b><span color="#ff79c6">/;s/..27m/<\/span><\/b>/')
  _todo=$(sed 's/\(- \[x\] \)\(.*\)/<span color="#ff79c6">\1<s>\2<\/s><\/span>/' <"$HOME"/.todo.md | sed 's/- \[[ |x]\] //')
  notify-send "  Calendar" "\n$_cal\n————————————————————\n$_todo" -r 9527
}

call_todo() {
  pid2=$(pgrep -f 'st -t status_util_todo')
  mx=$(xdotool getmouselocation --shell | grep X= | sed 's/X=//')
  my=$(xdotool getmouselocation --shell | grep Y= | sed 's/Y=//')
  if [[ ! "" = "$pid2" ]]; then
    kill "$pid2"
  else
    st -t status_util_todo -g 80x15+$((mx - 200))+$((my + 20)) -c FGN -e vim ~/.todo.md
  fi
}

click() {
  case "$1" in
  L) notify ;;
  R) call_todo ;; # todo 使用kde日历
  esac
}

case "$1" in
click) click "$2" ;;
notify) notify ;;
*) update ;;
esac
