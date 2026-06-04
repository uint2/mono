xset r rate 200 40

# picom 12.5 is sufficient, which can be found in Debian 13's APT as of time
# of writing.
picom --frame-opacity=1.0 --backend xrender &

feh --bg-scale ~/.local/share/wall.jpg &

# exec /usr/local/bin/kopiwm
exec /usr/local/bin/dwm
