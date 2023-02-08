# DWM YES

fork 自博主yaoccc  https://github.com/yaocccc/dwm

首先要感谢yaoccc的dwm,主要基于提交`e49d3b8`历史版本进行了修改,后续如果有比较大的新功能本仓库也会跟进



## 本仓库修改内容

**相比原版修改主要内容为:**

> 1. 彻底修复窗口隐藏和修复的bug 
>
>    之前有提过pr,但是后面发现scratchpad仍然会打断窗口恢复，现在这个仓库已经彻底修复，等有时间再去yaoccc那提个pr修复一下
>
> 2. 解决tag下没有窗口除了scratchpad仍然显示tag的bug 
>
>    比如tag2下打开scratchpad,但tag2没有别的其它窗口,状态栏仍然会显示tag2的图标
>
> 3. 增加功能:在其他符合规则的tag下打开内容,不会自动移动 
>
>    比如chrome固定在tag6,在tag1打开chrome不会自动移动tag6
>
> 4. 增加功能:增加可以不允许普通kill掉程序
>
>    比如使用tmux打开很多终端,为了防止手贱误关闭程序,可以把tmux加入到不允许普通kill保护中,当然仍然允许使用forcekill关闭程序;又或者打开腾讯会议共享桌面,开会等关键时刻,防止手贱把腾讯会议关了等等使用场景
>
> 5. 增加功能:原生支持键盘操作音量,屏幕亮度调整等
>
> 6. 增加了一些补丁,包括但不限于: 
>
>    - 连续两次激活按键关闭dwm才进行关闭,防止误触
>    - 热重启dwm 更改配置文件重新编译安装后可以直接重启dwm并保留当前已经打开窗口
>    - 增加新布局,但是无法使用窗口间距,等待手动添加修复,暂时还没启用新布局
>    - 旋转堆栈 可以更改窗口显示顺序
>



**其他一些次要修改内容为:**

> 1. 终端使用全部使用Alacritty,原版为st
> 2. statusbar修改较大,主要是为了符合自己的操作习惯



## 使用注意事项

1. dwm部分基本没有需要注意的只需要注意config.def.h文件中相关命令安装了对应的包就行\

   测试方法为:手动复制命令到终端中执行,如果成功就没问题



2. 主要可能问题点在statusbar

   同样也是命令执行问题,一定要安装了对应的包,否则可能会出现意外情况导致崩溃

   推荐先一个脚本一个脚本加入,不要上来全部执行.

   另外pacman执行sudo不需要输入密码,在`/etc/sudoers`加入`${user} ALL=(ALL) NOPASSWD: /usr/bin/pacman`



## 目前本仓库一些暂存bug

1. statusbar中音量功能中加入了一个显示蓝牙设备剩余电量功能,但是目前没法稳定使用,一般在刚开始连接时可以正常检测到,后面就不行. 这个暂时解决不了,只能说暂时不使用功能,需要等待上游更新或arch内核更新. (arch的蓝牙经常不稳定)

