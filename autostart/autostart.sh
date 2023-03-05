#! /bin/sh

# If you find fcitx5 icon is located at the most left of the straybar, please increase the delay value
sleep 1 # need to wait dwm start complete and fcitx5 start complete

# Notice that cron need exec before other program
cron() {
    let i=1
    while true; do
        [ $((i % 3)) -eq 0 ] && ~/.dwm/autostart/autoscreen.sh # check screen and autoset
        sleep 1; let i+=1
    done
}
cron&


# cfw & # clash for windows
crow & # translate
blueman-manager & # bluetooth manager
copyq & # copy software

picom_need_experimental=$(picom --help | grep -c experimental-backends) # 开启picom
if [ "$picom_need_experimental" -ge 1 ]; then
picom --experimental-backends --config  ~/.dwm/.config/picom.conf &
else
picom --config  ~/.dwm/.config/picom.conf &
fi


pkill -f statusbar.py
python3 ~/.dwm/statusbar/statusbar.py cron &>/dev/null


# libinput-gestures-setup start # touchpad open gesture
# xinput --set-prop 15 'libinput Accel Speed' 0.5 # set touchpad sensitivity

