#!/bin/sh

TIMEOUT=10

TMPDIR=$(mktemp)

cleanup() {
  rm -rf $TMPDIR
}
trap cleanup EXIT INT

smash() {
  sleep $TIMEOUT
  killall kopiwm
}

rm -rf $TMPDIR
mkdir -p $TMPDIR

VALGRIND="valgrind --leak-check=full --show-leak-kinds=all --"
VALGRIND="valgrind --leak-check=full --show-reachable=no --"
VALGRIND="valgrind --"

echo 'picom --frame-opacity=1.0 --backend xrender &' >$TMPDIR/xinitrc
echo 'feh --bg-fill ~/.local/share/wall.jpg &' >>$TMPDIR/xinitrc
echo "exec $VALGRIND ~/.local/bin/kopiwm" >>$TMPDIR/xinitrc

cat $TMPDIR/xinitrc

# smash &

XINITRC=$TMPDIR/xinitrc startx -- -keeptty >stdout.log 2>stderr.log
