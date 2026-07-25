xset r rate 220 40

# picom 12.5 is sufficient, which can be found in Debian 13's APT as of time
# of writing.
picom --frame-opacity=1.0 --backend xrender &

feh --bg-fill ~/.local/share/wall.jpg &

exec ~/.local/bin/kopiwm
# exec /usr/local/bin/dwm
