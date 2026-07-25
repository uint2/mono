function _G._statusline_filepath()
  local fp = vim.fn.expand('%')
  local non_empty = fp ~= nil and #fp > 0
  if non_empty then return fp end
  return ''
end

local set_line = function(branch)
  branch = '%#StatusLineBranch#' .. branch .. '%#StatusLine#'
  vim.opt.statusline = '%{%v:lua._statusline_filepath()%} %h%w%m%r '
    .. branch
    .. '%=+ '
  -- see "h: statusline" for more info.
end

require('brew.git-branch').init(set_line)
