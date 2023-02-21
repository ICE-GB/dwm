#!/usr/bin/env python

import os
import sys
import subprocess
import re
import time
import common

text_color="^c#000000^^b#ffffff0x99^"
icon_color="^c#000000^^b#ffffff0x99^"

filename= os.path.basename(__file__)
name=re.sub("\..*",'',filename)

def update(loop=False,exec=True):
  while True :
    icon=""
    text=time.strftime(" %H:%M:%S ", time.localtime())
    txt="^s"+str(name)+"^"+str(icon_color)+str(icon)+str(text_color)+str(text)
    common.write_to_file(txt+"\n",str(name))
    if loop == False : 
      if exec==True :
        os.system("xsetroot -name '"+str(txt)+"'")
      break
    time.sleep(1)

def notify(str='') :
  cmd = "notify-send "+'"'+" Calendar"+'"'+' "'+"\\nData: $(date '+%y-%m-%d \\nTime: %T')"+'"'+"  -r 9540"
  os.system(cmd)
  pass

def click(str='') :
  match str:
    case 'L':
      notify()
      pass
    case 'M':
      pass
    case 'R':
      pass
    case 'U':
      pass
    case 'D':
      pass
    case  _: pass

if __name__ == "__main__":
  if len(sys.argv) > 1:
    if(sys.argv[1]=="update") :
      pass
    else :
      click(sys.argv[1])
      update(exec=False)
  else :
    update()
   
