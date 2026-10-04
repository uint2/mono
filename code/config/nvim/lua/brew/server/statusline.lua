local omega = require('omega')

function _G._statusline_filepath()
  local fp = vim.fn.expand('%')
  local non_empty = fp ~= nil and #fp > 0
  if not non_empty then return '' end
  fp = omega.path_display(nil, fp)
  return '  ' .. fp
end

local set_line = function(branch)
  branch = '%#StatusLineBranch#' .. branch .. '%#StatusLine#'
  -- vim.opt.statusline = ' %f %h%w%m%r ' .. branch .. '%=+ '
  vim.opt.statusline = '%{%v:lua._statusline_filepath()%} %h%w%m%r '
    .. branch
    .. '%=+ '
end

require('brew.git-branch').init(set_line)
