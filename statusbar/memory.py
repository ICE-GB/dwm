#!/usr/bin/env python

import os
import sys
import subprocess
import re
import time
from typing import Tuple
import common

import psutil

icon_color="^c#3B001B^^b#ffb6c10x88^"
text_color="^c#3B001B^^b#ffb6c10x99^"
DELAY_TIME=1

filename= os.path.basename(__file__)
name=re.sub("\..*",'',filename)


def update(loop=False):
  while True :
    icon=" ﬘"
    text=str(int(psutil.virtual_memory()[2]))+"% "
    txt="^s"+str(name)+"^"+str(icon_color)+str(icon)+str(text_color)+str(text)
    common.write_to_file(txt+"\n",str(name))
    if loop == False : 
      os.system("xsetroot -name '"+str(txt)+"'")
      break
    time.sleep(DELAY_TIME)


def click(str='') :
  match str:
    case 'L':
      pass
    case 'M':
      pass
    case 'R':
      pass
      os.system("alacritty -t statusutil --class floatingTerminal -e btop")
    case 'U':
      pass
    case 'D':
      pass
    case  _: pass

def notify(str='') :
  pass

if __name__ == "__main__":
  if len(sys.argv) > 1:
    if(sys.argv[1]=="update") :
      pass
    else :
      click(sys.argv[1])
  else :
    update()
   
