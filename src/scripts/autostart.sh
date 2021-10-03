#!/bin/bash

function run {
  if ! pgrep $1 ;
  then
    $@&
  fi
}

#Set your native resolution IF it does not exist in xrandr
#More info in the script
#run $HOME/.xmonad/scripts/set-screen-resolution-in-virtualbox.sh

#cursor active at boot
xsetroot -cursor_name left_ptr &

# update monitors
autorandr --change

# starting utility applications at login time
/home/ajrae/.config/polybar/launch.sh
# run polybar mainbar-xmonad &
# run nm-applet &
# run pamac-tray &
# run xfce4-power-manager &
# run volumeicon &
# numlockx on &
# blueberry-tray &
picom --config $HOME/.xmonad/scripts/picom.conf &
/usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1 &
/usr/lib/xfce4/notifyd/xfce4-notifyd &

# starting user applications at login time
nitrogen --restore &
run /usr/bin/emacs --daemon &
