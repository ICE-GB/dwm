#!/usr/bin/env bash

pkill -f statusbar.py

python3 ~/.dwm/statusbar/statusbar.py cron &
