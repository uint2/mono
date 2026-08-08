if binary_exists git-checkout4; then
  gco() {
    local TARGET=$($GIT checkout4 $@)
    local EC=$?
    if [ $EC -eq 61 ]; then
      cd $TARGET
      return 0
    elif [ $EC -eq 62 ]; then
      # Get a string before first model ${VARNAME%%model*}
      # Get a string after first model  ${VARNAME#*model}
      #
      # https://git-scm.com/docs/git-check-ref-format
      # ╰── ':' character not allowed in branch, so we use it as the delimiter.
      cd ${TARGET#*:} && git checkout ${TARGET%%:*}
      return 0
    fi
    return $EC
  }
else
  alias gco="$GIT checkout"
fi
