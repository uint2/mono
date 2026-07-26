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

echo 'picom --frame-opacity=1.0 --backend xrender &' >$TMPDIR/xinitrc
echo 'feh --bg-fill ~/.local/share/wall.jpg &' >>$TMPDIR/xinitrc
echo "exec ${KOPIWM_BINARY}" >>$TMPDIR/xinitrc

cat $TMPDIR/xinitrc

# smash &
XINITRC=$TMPDIR/xinitrc startx -- -keeptty >stdout.log 2>stderr.log
