#include <X11/XF86keysym.h>

//=============================================================================
//                基础配置，看自己习惯
//=============================================================================
static const int newclientathead          = 0;         /* 定义新窗口在栈顶还是栈底 */
static int showsystray                    = 1;         /* 是否显示托盘栏 */
static const unsigned int systraypinning  = 0;         /* 托盘跟随的显示器 0代表不指定显示器 */
static const unsigned int systrayspacing  = 1;         /* 托盘间距 */
static const unsigned int systrayspadding = 5;         /* 托盘和状态栏的间隙 */
static const unsigned int borderpx        = 2;         /* 窗口边框大小 */
static int gappi                          = 6;         /* 窗口与窗口 缝隙大小 */
static int gappo                          = 6;         /* 窗口与边缘 缝隙大小 */
static const int _gappo                   = 6;         /* 窗口与窗口 缝隙大小 不可变 用于恢复时的默认值 */
static const int _gappi                   = 6;         /* 窗口与边缘 缝隙大小 不可变 用于恢复时的默认值 */
static const int vertpad                  = 3;         /* vertical padding of bar */
static const int sidepad                  = 3;         /* horizontal padding of bar */
static const int overviewgappi            = 20;        /* overview时 窗口与窗口 缝隙大小 */
static const int overviewgappo            = 30;        /* overview时 窗口与边缘 缝隙大小 */
static const int showbar                  = 1;         /* 是否显示状态栏 */
static const int topbar                   = 1;         /* 指定状态栏位置 0底部 1顶部 */
static const float mfact                  = 0.5;       /* 主工作区 大小比例 */
static const int   nmaster                = 1;         /* 主工作区 窗口数量 */
static const int nstack                   = 0;         /* number of clients in primary stack area */
static const unsigned int snap            = 10;        /* 边缘依附宽度 */


//=============================================================================
//                 快速切换历史tag 
//=============================================================================
#define RESTORE_TAG_MAXNUM 15 // 用于恢复时存储的最多记录tag数，一般不需要更改，必须大于等于1
#define SWITCH_TAG_LOOP 1    // 是否支持循环（穿越）,1为支持
// 如果习惯是单按键切换最近的两个窗口，则设置 RESTORE_TAG_MAXNUM 为2 , SWITCH_TAG_LOOP 为1


//=============================================================================
// 字体配置
//=============================================================================
static const char *fonts[]               = {
    "JetBrainsMono Nerd Font:style=medium:size=13", 
    "monospace:size=13",
	"Monaco:style=Regular:size=11",
	"Symbols Nerd Font:style=2048-em:size=17",
	"Microsoft YaHei:size=11:style=Regular:antialias=true:autohint:true",
	"JoyPixels:size=13:antialias=true:autohint=true"
};  


//=============================================================================
/* 颜色设置 ColFg, ColBg, ColBorder */ 
//=============================================================================
#include "themes/nord.h"
static const char *colors[][3] = {        
    [SchemeNorm] = { "#ffffff", "#333333", "#444444" },
    [SchemeSel] = { red2, "#47575F", "#f09a7f" }, // #abd687
    [SchemeSelGlobal] = { "#ffffff", "#47575F", "#fcf86f" },
    [SchemeTabSel] = { red2,    black,  black },
    [SchemeTabNorm]= { white,   black,  black },
    [SchemeUnderline] = { red2, black, black }, 
    [SchemeMode]= { green,   black,  black },
    [SchemeHid] = { "#dddddd", NULL, NULL },
    [SchemeSystray] = { NULL, blue, NULL },
    [SchemeNormTag] = { "#aaaaaa", "#333333", NULL },
    [SchemeSelTag] = { "#eeeeee", "#333333", NULL },
    [SchemeBarEmpty] = { "#1e222a", "#1e222a", NULL },
    [SchemeOverView] = { red2, black, black },
};
//-----------------------------------------------------------------------------
static int statusbar_h_bias=13;
static int tag_line_h=7;
//-----------------------------------------------------------------------------
/* 透明度设置 ColFg, ColBg, ColBorder */ 
static const unsigned int baralpha        = 0xc0;      /* 状态栏透明度 */
static const unsigned int borderalpha     = 0xdd;      /* 边框透明度 */
static const unsigned int alphas[][3] = {         
    // [SchemeNorm] = { OPAQUE, baralpha, borderalpha }, 
    // [SchemeSel] = { OPAQUE, baralpha, borderalpha },
    // [SchemeSelGlobal] = { OPAQUE, baralpha, borderalpha },
    // [SchemeNormTag] = { OPAQUE, baralpha, borderalpha }, 
    // [SchemeSelTag] = { OPAQUE, baralpha, borderalpha },
    // [SchemeBarEmpty] = { NULL, 0xa0a, NULL },
    // [SchemeStatusText] = { OPAQUE, 0x88, NULL },
    [SchemeNorm] = { NULL, 0xff, borderalpha }, 
    [SchemeSel] = { NULL, 0xff, borderalpha },
    [SchemeSelGlobal] = { NULL, 0xff, borderalpha },
    [SchemeTabSel] = { NULL, 0xff, borderalpha },
    [SchemeTabNorm]= { NULL, 0xff, borderalpha },
    [SchemeOverView] = { 0xff, 0xff, borderalpha },
    [SchemeUnderline] = { 0xff, 0xff, borderalpha }, 
    [SchemeMode]= { NULL, 0xff, borderalpha },

    [SchemeNormTag] = { NULL, 0xff, NULL }, 
    [SchemeSelTag] = { NULL, 0xff, NULL },
    [SchemeBarEmpty] = { 0xff, 0xff, NULL },
    [SchemeStatusText] = { NULL, 0xff, NULL },
    // [SchemeSystray] = { NULL, 0xff, NULL },
};


//=============================================================================
/* 防止误关闭，一些程序关闭快捷键不会响应 */
//=============================================================================
static const char *disablekillclient[] = {
  "wemeetapp", // 腾讯会议顶栏,防止开会时关闭桌面共享了，没有这个需求可以注释
  "tmux", // tmux不要误关了，防止有子窗口还在运行
  "QQ", // QQ关闭后会直接退出,不会最小化,微信不需要这个操作
};

//=============================================================================
/* 使用killclient关闭窗口默认执行隐藏操作 */
// 配置 ToggleShowHideWindows 使用
//=============================================================================
static const char *showhidewindows[] = {
  "WeChat",
  "钉钉",
  "QQ",
};


//=============================================================================
//                      自启动脚本
//=============================================================================
static const char *autostartscript = "~/.dwm/autostart/autostart.sh";
//-----------------------------------------------------------------------------
//                     状态栏启动脚本
static const char *statusbarscript = "~/.dwm/statusbar/statusbar.py";//gxt_kt



//=============================================================================
//                 自定义 scratchpad
//=============================================================================
static const char scratchpadname[] = "scratchpad";


//=============================================================================
//                   tag icon 
//            ﮸  ﭮ 切  ﬐ // 增补图标
//            
//=============================================================================
static const char *tags[] = { "", "", "", "", "", "󰤚","", "","","" };


//=============================================================================
// 以下为临时文件，判断启动状态或相关的，一般不用动
// 如果发现启动异常，相关脚本或任务没有执行，可以删除这些临时文件再重启电脑
//=============================================================================
/* Lockfile */ // doublepressquitPatch
static char lockfile[] = "/tmp/dwm.lock"; 
//-----------------------------------------------------------------------------
/* Super-Ctrl-Shift-Esc 热重启dwm后不会重复执行autostart脚本 */
static const char* avoid_repeat_auto_start = "/tmp/dwm_avoid_repeat_auto_start.lock"; // doublepressquitPatch
//-----------------------------------------------------------------------------
// restore after restart
#define SESSION_FILE "/tmp/dwm-session"
#define SESSION_TAG_FILE "/tmp/dwm-tag-session"


//=============================================================================
// 规则设置
// 自定义窗口显示规则 
// class instance title 主要用于定位窗口适合哪个规则 
// tags mask 定义符合该规则的窗口的tag 0 表示当前tag 
// isfloating 定义符合该规则的窗口是否浮动 
// isglobal 定义符合该规则的窗口是否全局浮动 
// isnoborder 定义符合该规则的窗口是否无边框 
// monitor 定义符合该规则的窗口显示在哪个显示器上 -1 为当前屏幕 
// floatposition 定义符合该规则的窗口显示的位置 0 中间，1到9分别为9宫格位置，例如1左上，9右下，3右上 
//=============================================================================
static const Rule rules[] = {
    /* class                 instance              title             tags mask     isfloating  isglobal    isnoborder monitor floatposition */
    /** 优先级高 越在上面优先度越高 */
    { NULL,                  NULL,                "图片查看器",      0,            1,          0,          0,        -1,      0}, // qq图片查看器        浮动
    { NULL,                  NULL,                "图片查看",        0,            1,          0,          0,        -1,      0}, // 微信图片查看器      浮动

    /** 普通优先度 */
    {"obs",                  NULL,                 NULL,             1 << 3,       0,          0,          0,        -1,      0}, // obs        tag -> 󰕧
    {"chrome",               NULL,                 NULL,             1 << 4,       0,          0,          0,        -1,      0}, // chrome     tag -> 
    {"Chromium",             NULL,                 NULL,             1 << 4,       0,          0,          0,        -1,      0}, // Chromium   tag -> 
    {"music",                NULL,                 NULL,             1 << 5,       1,          0,          1,        -1,      0}, // music      tag ->  浮动、无边框
    { NULL,                 "qq",                  NULL,             1 << 6,       0,          0,          1,        -1,      0}, // qq         tag -> ﬄ 无边框
    { NULL,                 "wechat.exe",          NULL,             1 << 7,       0,          0,          1,        -1,      0}, // wechat     tag -> ﬐ 无边框
    { NULL,                 "wxwork.exe",          NULL,             1 << 8,       0,          0,          1,        -1,      0}, // workwechat tag ->  无边框
    {"Vncviewer",            NULL,                 NULL,             0,            1,          0,          1,        -1,      2}, // Vncviewer           浮动、无边框 屏幕顶部
    {"flameshot",            NULL,                 NULL,             0,            1,          0,          0,        -1,      0}, // 火焰截图            浮动
    {"scratchpad",          "scratchpad",         "scratchpad",      TAGMASK,      1,          1,          1,        -1,      2}, // scratchpad          浮动、全局、无边框 屏幕顶部
    {"Pcmanfm",              NULL,                 NULL,             0,            1,          0,          1,        -1,      3}, // pcmanfm             浮动、无边框 右上角
    {"wemeetapp",            NULL,                 NULL,             TAGMASK,      1,          1,          0,        -1,      0}, // !!!腾讯会议在切换tag时有诡异bug导致退出 变成global来规避该问题

    /** 部分特殊class的规则 */
    {"float",                NULL,                 NULL,             0,            1,          0,          0,        -1,      0}, // class = float       浮动
    {"global",               NULL,                 NULL,             TAGMASK,      0,          1,          0,        -1,      0}, // class = gloabl      全局
    {"noborder",             NULL,                 NULL,             0,            0,          0,          1,        -1,      0}, // class = noborder    无边框
    {"FGN",                  NULL,                 NULL,             TAGMASK,      1,          1,          1,        -1,      0}, // class = FGN         浮动、全局、无边框
    {"FG",                   NULL,                 NULL,             TAGMASK,      1,          1,          0,        -1,      0}, // class = FG          浮动、全局
    {"FN",                   NULL,                 NULL,             0,            1,          0,          1,        -1,      0}, // class = FN          浮动、无边框
    {"GN",                   NULL,                 NULL,             TAGMASK,      0,          1,          1,        -1,      0}, // CLASS = GN          全局、无边框

    /** 优先度低 越在上面优先度越低 */
    { NULL,                  NULL,                "crx_",            0,            1,          0,          0,        -1,      0}, // 错误载入时 会有crx_ 浮动
    { NULL,                  NULL,                "broken",          0,            1,          0,          0,        -1,      0}, // 错误载入时 会有broken 浮动
};


//=============================================================================
// overview : win+tab
//=============================================================================
static const char *overviewtag = "OVERVIEW";
static const Layout overviewlayout = { "", overview };


//=============================================================================
/* 自定义布局
 * 有两套布局:
 *   第一套是基于yaoccc的，我写了一个tile_right做补充
 *   第二套是基于flextile布局做的更改加补充
 *
 * 建议第一次使用使用第一套布局，熟悉以后采用第二套
 * 第一套大多数情况够用了，入门第一套足够了
 * 第二套相对第一套做了很多补充，完全包含了第一套的内容
 * 第二套布局过多，很多都用不到，建议用不到的注释掉，方便快速切换想要的布局
 */
//=============================================================================
#if 0 // 0为第一套，1第二套
//-----------------------------------------------------------------------------
static const Layout layouts[] = {
  //symbol     arrange function   
    {"﬿",        tile},      /* 主次栈 */
    {"﩯",     magicgrid},   /* 网格 */
    {"TR",    tile_right},   /* 主次栈 主侧放在右侧*/
    {NULL,       NULL} //最后一个需要是NULL,NULL,cyclelayout,请勿更改
};
//-----------------------------------------------------------------------------
#else
//-----------------------------------------------------------------------------
static const Layout layouts[] = {
	/* symbol     arrange function, { nmaster, nstack, layout, master axis, stack axis, secondary stack axis } */
	{ "﬿",          flextile,         { -1, -1, SPLIT_VERTICAL, TOP_TO_BOTTOM, TOP_TO_BOTTOM, 0, NULL } }, // default tile layout
  { "﩯",        magicgrid,{0} },    /* 网格 */
	// { "><>",      NULL,             {0} },    /* no layout function means floating behavior */ // 已经被win+shift+f 替代，不用这个
	// { "[M]",      flextile,         { -1, -1, NO_SPLIT, MONOCLE, MONOCLE, 0, NULL } }, // monocle
	// { "|||",      flextile,         { -1, -1, SPLIT_VERTICAL, LEFT_TO_RIGHT, TOP_TO_BOTTOM, 0, NULL } }, // columns (col) layout //其实就是tile
	// { ">M>",      flextile,         { -1, -1, FLOATING_MASTER, LEFT_TO_RIGHT, LEFT_TO_RIGHT, 0, NULL } }, // floating master
	// { "[D]",      flextile,         { -1, -1, SPLIT_VERTICAL, TOP_TO_BOTTOM, MONOCLE, 0, NULL } }, // deck
	{ "TTT",      flextile,         { -1, -1, SPLIT_HORIZONTAL, LEFT_TO_RIGHT, LEFT_TO_RIGHT, 0, NULL } }, // bstack
	{ "===",      flextile,         { -1, -1, SPLIT_HORIZONTAL, LEFT_TO_RIGHT, TOP_TO_BOTTOM, 0, NULL } }, // bstackhoriz
	// { ":::",      flextile,         { -1, -1, SPLIT_HORIZONTAL, LEFT_TO_RIGHT, TOP_TO_BOTTOM, 0, monoclesymbols } }, // centeredmaster
	{ "gapless",      flextile,         { -1, -1, NO_SPLIT, GAPPLESSGRID, GAPPLESSGRID, 0, NULL } }, // gappless grid
	{ "[\\]",     flextile,         { -1, -1, NO_SPLIT, DWINDLE, DWINDLE, 0, NULL } }, // fibonacci dwindle
	{ "(@)",      flextile,         { -1, -1, NO_SPLIT, SPIRAL, SPIRAL, 0, NULL } }, // fibonacci spiral
	{ "[T]",      flextile,         { -1, -1, SPLIT_VERTICAL, LEFT_TO_RIGHT, TATAMI, 0, NULL } }, // tatami mats
	{ NULL,       NULL,             {0} },//最后一个需要是NULL,NULL,cyclelayout,请勿更改
 };
#endif


//=============================================================================
//                          不需要更改
//=============================================================================
#define SHCMD(cmd) { .v = (const char*[]){ "/bin/sh", "-c", cmd, NULL } }
#define MODKEY Mod4Mask  // Super
#define TAGKEYS(KEY, TAG, cmd) \
    { MODKEY,              KEY, view,       {.ui = 1 << TAG, .v = cmd} }, \
    { MODKEY|ShiftMask,    KEY, tag,        {.ui = 1 << TAG} }, \
    { MODKEY|ControlMask,  KEY, toggleview, {.ui = 1 << TAG} }, \



//=============================================================================
//                           按键配置
//  modifier            key              function          argument 
//=============================================================================
static Key keys[] = {
//=============================================================================
//           一些基础快捷键，符合配置的核心按键思想，不建议更改
//=============================================================================
  	{ MODKEY,              XK_h,       focusdir,          {.i = 0 } },  // 切换聚焦窗口
  	{ MODKEY,              XK_j,       focusdir,          {.i = 1 } },  // 切换聚焦窗口
  	{ MODKEY,              XK_k,       focusdir,          {.i = 2 } },  // 切换聚焦窗口
  	{ MODKEY,              XK_l,       focusdir,          {.i = 3 } },  // 切换聚焦窗口
//-----------------------------------------------------------------------------
    { MODKEY|ShiftMask,    XK_h,       ExchangeClient,    {.i = 0} },   // 移动窗口
    { MODKEY|ShiftMask,    XK_j,       ExchangeClient,    {.i = 1 } },  // 移动窗口
    { MODKEY|ShiftMask,    XK_k,       ExchangeClient,    {.i = 2 } },  // 移动窗口
    { MODKEY|ShiftMask,    XK_l,       ExchangeClient,    {.i = 3} },   // 移动窗口
//-----------------------------------------------------------------------------
    { MODKEY,              XK_Tab,     toggleoverview,    {0} },        // 显示所有tag 或 跳转到聚焦窗口的tag */
    { Mod1Mask,            XK_Tab,     focusstack,        {.i = +1} },  // 本tag内切换聚焦窗口 
    { Mod1Mask|ShiftMask,  XK_Tab,     focusstack,        {.i = -1} },  // 本tag内切换聚焦窗口  
//-----------------------------------------------------------------------------
    { MODKEY,              XK_f,       togglefloating,    {0} },        // 开启/关闭 当前窗口的float模式
    { MODKEY|ShiftMask,    XK_f,       toggleallfloating, {0} },        // 开启/关闭 当前tag 的float模式
    { MODKEY,              XK_g,       toggleglobal,      {0} },        // 开启/关闭 全局 
//-----------------------------------------------------------------------------
    { MODKEY,              XK_comma,   setmfact,          {.f = -0.05} }, // 缩小主工作区
    { MODKEY,              XK_period,  setmfact,          {.f = +0.05} }, // 放大主工作区
//-----------------------------------------------------------------------------
    { MODKEY,              XK_q,       killclient,        {0} },        // 关闭当前窗口
    { MODKEY|ShiftMask,    XK_q,       forcekillclient,   {0} },        // 强制关闭当前窗口
    { MODKEY|ControlMask,  XK_q,       forcekillclient,   {0} },        // 强制关闭当前窗口
    { MODKEY|ShiftMask,    XK_Escape,  quit,              {0} },        // 退出dwm
    { MODKEY|ControlMask|ShiftMask,XK_Escape, quit,       {1} },        // 重启dwm 
//-----------------------------------------------------------------------------
    { MODKEY,              XK_o,       GoBackToPreTag,    {0} },        // 切换历史tag
    { MODKEY,              XK_i,       GoBackToNextTag,   {0} },        // 切换历史tag 
//-----------------------------------------------------------------------------
  

//=============================================================================
//                 一些其它快捷键，可以根据需要和习惯更改
//=============================================================================
    { MODKEY,              XK_d,       hidewin,           {0} },          // 隐藏窗口
    { MODKEY|ShiftMask,    XK_d,       restorewin,        {0} },          // 取消隐藏窗口
//-----------------------------------------------------------------------------
    { MODKEY,              XK_z,       showonlyorall,     {0} },          // 单窗口
    { MODKEY,              XK_F11,     fullscreen,        {0} },          // 开启/关闭 全屏
//-----------------------------------------------------------------------------
    { MODKEY,              XK_b,       togglebar,         {0} },          // 开启/关闭 状态栏 
    { MODKEY,              XK_backslash,togglesystray,    {0} },          // 开启/关闭 托盘栏
//-----------------------------------------------------------------------------
    { MODKEY|ControlMask,  XK_equal,   setgap,            {.i = +5} },    // gap增大
    { MODKEY|ControlMask,  XK_minus,   setgap,            {.i = -5} },    // gap减小
    { MODKEY|ControlMask,  XK_BackSpace,setgap,           {.i = 0} },     // gap重置
//-----------------------------------------------------------------------------
    { MODKEY,              XK_Up,           focusstack,       {.i = -1} },               /* super up           |  本tag内切换聚焦窗口 */
    { MODKEY,              XK_Down,         focusstack,       {.i = +1} },               /* super down         |  本tag内切换聚焦窗口 */
    { MODKEY,              XK_Left,         viewtoleft,       {0} },                     /* super left         |  聚焦到左边的tag */
    { MODKEY,              XK_Right,        viewtoright,      {0} },                     /* super right        |  聚焦到右边的tag */ 
    { MODKEY|ControlMask,  XK_Left,         tagtoleft,        {0} },                     /* super shift left   |  将本窗口移动到左边tag */
    { MODKEY|ControlMask,  XK_Right,        tagtoright,       {0} },                     /* super shift right  |  将本窗口移动到右边tag */
    // { MODKEY,              XK_Up,      movewin,           {.ui = UP} },   // 移动窗口
    // { MODKEY,              XK_Down,    movewin,           {.ui = DOWN} }, // 移动窗口
    // { MODKEY,              XK_Left,    movewin,           {.ui = LEFT} }, // 移动窗口
    // { MODKEY,              XK_Right,   movewin,           {.ui = RIGHT} },// 移动窗口  
    // { MODKEY|ControlMask,  XK_Up,      resizewin,         {.ui = V_REDUCE} },// 调整窗口 
    // { MODKEY|ControlMask,  XK_Down,    resizewin,         {.ui = V_EXPAND} },// 调整窗口      
    // { MODKEY|ControlMask,  XK_Left,    resizewin,         {.ui = H_REDUCE} },// 调整窗口     
    // { MODKEY|ControlMask,  XK_Right,   resizewin,         {.ui = H_EXPAND} },// 调整窗口    
//-----------------------------------------------------------------------------
    { MODKEY|ShiftMask,    XK_Return,  zoom,              {0} },          // 将当前聚焦窗口置为主窗口
//-----------------------------------------------------------------------------
  

//=============================================================================
//                      基础和flextile 布局相关
//=============================================================================
    { MODKEY,              XK_a,       incnmaster,        {.i = +1} },    // 改变主窗口数 1或2 都有效
	  { MODKEY|ShiftMask,    XK_comma,   cyclelayout,       {.i = -1 } },   // 循环布局 都有效
	  { MODKEY|ShiftMask,    XK_period,  cyclelayout,       {.i = +1 } },   // 循环布局 都有效
//-----------------------------------------------------------------------------
    { MODKEY,          XK_bracketleft, incnstack,         {.i = -1 } },   // 增加从堆栈数 仅flextile有效
    { MODKEY,          XK_bracketright,incnstack,         {.i = +1 } },   // 减少从堆栈数 仅flextile有效
  	{ MODKEY|ControlMask,  XK_Return,  mirrorlayout,      {0} },          // 翻转主区域和堆栈区域 仅flextile有效
//-----------------------------------------------------------------------------
	  { MODKEY|ControlMask,  XK_comma,   rotatelayoutaxis,  {.i = -1 } },   // 循环另一种布局 仅flextile有效
	  { MODKEY|ControlMask,  XK_period,  rotatelayoutaxis,  {.i = +1 } },   // 循环另一种布局 仅flextile有效
//-----------------------------------------------------------------------------
    // It's just need to map one key to change layout between layouts[0] and layouts[1].
    // { MODKEY|ShiftMask,  XK_o,      selectlayout,     {.v = &layouts[0]} }, // 切换到第1个布局 
    // { MODKEY|ControlMask,XK_o,      selectlayout,     {.v = &layouts[1]} }, // 切换到第2个布局 
//-----------------------------------------------------------------------------


//=============================================================================
//                              多显示器配置
//=============================================================================
    { MODKEY|Mod1Mask,     XK_Left,     focusmon,         {.i = -1} },     // 光标移动到另一个显示器 
    { MODKEY|Mod1Mask,     XK_Right,    focusmon,         {.i = +1} },     // 光标移动到另一个显示器
    { MODKEY|Mod1Mask,     XK_h,        focusmon,         {.i = -1} },     // 光标移动到另一个显示器
    { MODKEY|Mod1Mask,     XK_l,        focusmon,         {.i = +1} },     // 光标移动到另一个显示器
    { MODKEY|ShiftMask,    XK_Left,     tagmon,           {.i = -1} },     // 将聚焦窗口移动到另一个显示器 
    { MODKEY|ShiftMask,    XK_Right,    tagmon,           {.i = +1} },     // 将聚焦窗口移动到另一个显示器   
//-----------------------------------------------------------------------------


//=============================================================================
//                              其它命令
//=============================================================================
    // Notice that if you first use copyq , Remeber config 1.disable tray show 2.Enable hidden mainwindow. Then you can use this better.
    // { MODKEY,              XK_v,        spawn,   SHCMD("copyq toggle") },  // copyq
    { MODKEY|ShiftMask,    XK_s,        spawn,   SHCMD("flameshot gui") }, // flameshot
    { MODKEY|ControlMask,  XK_l,        spawn,   SHCMD("~/.dwm/i3lock/lock.sh") },   
    { MODKEY,              XK_e,        spawn,   SHCMD("kitty -e ranger") }, // 打开资源管理器
    { MODKEY,              XK_BackSpace,spawn,   SHCMD("playerctl play-pause") },// audio play/pause

/* spawn + SHCMD 执行对应命令(已下部分建议完全自己重新定义) */
    { MODKEY,              XK_s,      togglescratch, SHCMD("st -t scratchpad -c float") },                      /* super s          | 打开scratch终端        */
    { MODKEY,              XK_Return, spawn, SHCMD("st -e fish") },                                             /* super enter      | 打开st终端             */
    { MODKEY,              XK_minus,  spawn, SHCMD("st -c FG -e fish") },                                       /* super -          | 打开全局st终端         */
    { MODKEY,              XK_space,  spawn, SHCMD("st -c float -e fish") },                                    /* super space      | 打开浮动st终端         */
    // { MODKEY,              XK_d,      spawn, SHCMD("~/scripts/call_rofi.sh run") },                             /* super d          | rofi: 执行run          */
    // { MODKEY|ShiftMask,    XK_d,      spawn, SHCMD("~/scripts/call_rofi.sh drun") },                            /* super shift d    | rofi: 执行drun         */
    { Mod1Mask,            XK_space,  spawn, SHCMD("~/.config/rofi/launchers/type-3/launcher.sh") },            /* alt space        | rofi: 执行drun         */
    // { MODKEY,              XK_p,      spawn, SHCMD("~/scripts/call_rofi.sh custom") },                          /* super p          | rofi: 执行自定义脚本   */
    // { MODKEY|ShiftMask,    XK_p,      spawn, SHCMD("~/scripts/call_rofi.sh window") },                          /* super shift p    | rofi: 执行window       */
    { MODKEY,              XK_v,      spawn, SHCMD("$DWM/.bin/clip_history.sh") },                              /* super v          | 剪切板历史             */
    { MODKEY|ShiftMask,    XK_a,      spawn, SHCMD("flameshot gui -c -p ~/Pictures/screenshots") },             /* super shift a    | 截图                   */
    // { MODKEY|ShiftMask,    XK_k,      spawn, SHCMD("~/scripts/screenkey.sh") },                                 /* super shift k    | 打开键盘输入显示       */
    // { MODKEY|ShiftMask,    XK_q,      spawn, SHCMD("kill -9 $(xprop | grep _NET_WM_PID | awk '{print $3}')") }, /* super shift q    | 选中某个窗口并强制kill */
    { ShiftMask|ControlMask, XK_c,    spawn, SHCMD("xclip -o | xclip -selection c") },                          /* super shift c    | 进阶复制               */
    { MODKEY,              XK_c,      spawn, SHCMD("google-chrome-stable --force-device-scale-factor=1.25") },  /* super c    | chrome               */
    // { MODKEY,              XK_i,      spawn, SHCMD("/home/gb/opt/idea-IU-213.5744.223/bin/idea.sh") },          /* super i    | idea               */
    { MODKEY,              XK_F1,     spawn, SHCMD("killall pcmanfm || pcmanfm") },                             /* super F1         | 打开/关闭pcmanfm       */
    { MODKEY,              XK_p,      spawn, SHCMD("$DWM/.bin/rofi.sh") },                                       /* super p          | rofi: 执行自定义脚本   */
    { MODKEY,              XK_n,      spawn, SHCMD("i3lock") },                                   /* super n          | 锁定屏幕               */
    { MODKEY|ShiftMask,    XK_Up,     spawn, SHCMD("$DWM/.bin/set_vol.sh up") },                                 /* super shift up   | 音量加                 */
    { MODKEY|ShiftMask,    XK_Down,   spawn, SHCMD("$DWM/.bin/set_vol.sh down") },                               /* super shift down | 音量减                 */
//-----------------------------------------------------------------------------
    // { MODKEY,              XK_q,  ToggleShowHideWindows,    {.v="QQ"} },          
    // { MODKEY,              XK_w,  ToggleShowHideWindows,    {.v="WeChat"} },     
    // { MODKEY,              XK_x,  ToggleShowHideWindows,    {.v="钉钉"} },      
//-----------------------------------------------------------------------------
    // { MODKEY|ShiftMask,      XK_q,    spawn, SHCMD("kill -9 $(xprop | grep _NET_WM_PID | awk '{print $3}')") },//选中某个窗口并强制kill
    // { ShiftMask|ControlMask, XK_c,    spawn, SHCMD("xclip -o | xclip -selection c") }, // 进阶复制


//=============================================================================
//                         其它一些api可以自行启用
//                   启用时记得先检查按键避免重复定义冲突
//=============================================================================
    // { MODKEY|ShiftMask,    XK_j,        rotatestack,      {.i = +1 } },    /* rotate the stack*/
    // { MODKEY|ShiftMask,    XK_k,        rotatestack,      {.i = -1 } },    /* rotate the stack*/
//-----------------------------------------------------------------------------
    // { MODKEY|ShiftMask,    XK_Left,     viewtoleft,       {0} },      //聚焦到左边的tag 
    // { MODKEY|ShiftMask,    XK_Right,    viewtoright,      {0} },      // 聚焦到右边的tag 
//-----------------------------------------------------------------------------
    // { MODKEY|ShiftMask,    XK_Left,     tagtoleft,        {0} },      // 将本窗口移动到左边tag
    // { MODKEY|ShiftMask,    XK_Right,    tagtoright,       {0} },      // 将本窗口移动到右边tag 
//-----------------------------------------------------------------------------




//=============================================================================
    /* super key : 跳转到对应tag (可附加一条命令 若目标目录无窗口，则执行该命令) */
    /* super shift key : 将聚焦窗口移动到对应tag */
    /* key tag cmd */
    /* 注意从0开始算，会错开一个窗口 */
//=============================================================================
    TAGKEYS(XK_1, 0,  0)
    TAGKEYS(XK_2, 1,  0)
    TAGKEYS(XK_3, 2,  0)
    TAGKEYS(XK_4, 3,  0)
    TAGKEYS(XK_5, 4,  0)
    TAGKEYS(XK_6, 5,  0)
    TAGKEYS(XK_7, 6,  0)
    TAGKEYS(XK_8, 7,  0)
    TAGKEYS(XK_9, 8,  0)
    TAGKEYS(XK_r, 5,  "obs")
    TAGKEYS(XK_c, 6,  "google-chrome-stable") 
    TAGKEYS(XK_m, 7,  "st ncmpcpp")
//-----------------------------------------------------------------------------
    //TAGKEYS(XK_0, 8,  "linuxqq")
    //TAGKEYS(XK_w, 9,  "/opt/apps/com.qq.weixin.deepin/files/run.sh")
    //TAGKEYS(XK_l, 10, "/opt/apps/com.qq.weixin.work.deepin/files/run.sh")
    
    
//=============================================================================
//    根据相关信号执行指令，除python脚本位置外一般不需要更改，但需要注意相关指令包存在
//=============================================================================
{ 0, XF86XK_AudioMute,         spawn, SHCMD("pamixer -t;  python3 /home/gb/.dwm/statusbar/vol.py notify ") },
{ 0, XF86XK_AudioRaiseVolume,  spawn, SHCMD("pamixer -i 5;python3 /home/gb/.dwm/statusbar/vol.py notify ") },
{ 0, XF86XK_AudioLowerVolume,  spawn, SHCMD("pamixer -d 5;python3 /home/gb/.dwm/statusbar/vol.py notify ") },
{ 0, XF86XK_AudioPause,        spawn, SHCMD("playerctl stop") },
{ 0, XF86XK_AudioPrev,         spawn, SHCMD("playerctl previous") },
{ 0, XF86XK_AudioNext,         spawn, SHCMD("playerctl next") },
{ 0, XF86XK_AudioPlay,         spawn, SHCMD("playerctl play") },
{ 0, XF86XK_AudioStop,         spawn, SHCMD("playerctl stop") },
{ 0, XF86XK_AudioStop,         spawn, SHCMD("playerctl stop") },
{ 0, XF86XK_MonBrightnessUp,   spawn, SHCMD("light -A 5; notify-send -r 9123 -h int:value:`light` -h string:hlcolor:#dddddd 'Backlight' " ) },
{ 0, XF86XK_MonBrightnessDown, spawn, SHCMD("light -U 5; notify-send -r 9123 -h int:value:`light` -h string:hlcolor:#dddddd 'Backlight' " ) },

};


//=============================================================================
// 按键操作和statusbar操作
// 以下内容一般不需要更改
//=============================================================================
static Button buttons[] = {
    /* click           event mask      button       function       argument  */
//=============================================================================
    /* 点击窗口标题栏操作 */
//=============================================================================
    { ClkWinTitle,         0,          Button1,     hideotherwins, {0} },                                   // 右键         |  点击标题     |  隐藏其他窗口仅保留该窗口
    { ClkWinTitle,         0,          Button3,     togglewin,     {0} },                                   // 左键         |  点击标题     |  切换窗口显示状态
//=============================================================================
    /* 点击窗口操作 */
//=============================================================================
    { ClkClientWin,        MODKEY,     Button1,     movemouse,     {0} },                                   // super+左键  |  拖拽窗口     |  拖拽窗口
    { ClkClientWin,        MODKEY,     Button3,     resizemouse,   {0} },                                   // super+右键  |  拖拽窗口     |  改变窗口大小
//=============================================================================
    /* 点击tag操作 */
//=============================================================================
    { ClkTagBar,           0,          Button1,     view,          {0} },                                   // 左键        |  点击tag      |  切换tag
	  { ClkTagBar,           0,          Button3,     toggleview,    {0} },                                   // 右键        |  点击tag      |  切换是否显示tag
    { ClkTagBar,           MODKEY,     Button1,     tag,           {0} },                                   // super+左键  |  点击tag      |  将窗口移动到对应tag
    { ClkTagBar,           0,          Button4,     viewtoleft,    {0} },                                   // 鼠标滚轮上  |  tag          |  向前切换tag
	  { ClkTagBar,           0,          Button5,     viewtoright,   {0} },                                   // 鼠标滚轮下  |  tag          |  向后切换tag

//=============================================================================
    /* 点击bar空白处 有需要自行启动 */
//=============================================================================
    // { ClkBarEmpty,         0,          Button1,     spawn,       SHCMD("~/scripts/call_rofi.sh window") },        // 左键        |  bar空白处    |  rofi 执行 window
    // { ClkBarEmpty,         0,          Button3,     spawn,       SHCMD("~/scripts/call_rofi.sh drun") },          // 右键        |  bar空白处    |  rofi 执行 drun

//=============================================================================
    /* 点击状态栏操作 */
 // 根据状态栏的信号执行  statusbarscript signal L/M/R/U/D
//=============================================================================
    { ClkStatusText,       0,          Button1,     clickstatusbar,{0} },   // 左键        |  点击状态栏   | 
    { ClkStatusText,       0,          Button2,     clickstatusbar,{0} },   // 中键        |  点击状态栏   |  根据状态栏的信号执行 ~/scripts/dwmstatusbar.sh $signal M
    { ClkStatusText,       0,          Button3,     clickstatusbar,{0} },   // 右键        |  点击状态栏   |  根据状态栏的信号执行 ~/scripts/dwmstatusbar.sh $signal R
    { ClkStatusText,       0,          Button4,     clickstatusbar,{0} },   // 鼠标滚轮上  |  状态栏       |  根据状态栏的信号执行 ~/scripts/dwmstatusbar.sh $signal U
    { ClkStatusText,       0,          Button5,     clickstatusbar,{0} },   // 鼠标滚轮下  |  状态栏       |  根据状态栏的信号执行 ~/scripts/dwmstatusbar.sh $signal D
};
