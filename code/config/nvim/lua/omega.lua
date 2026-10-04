local M = {}
local path_display_cache = {}

M.path_display = function(opts, path)
  if path == nil then return '' end
  local p = path_display_cache[path]
  if p ~= nil then return p end
  p = path
  p = string.gsub(p, '/Users/nvkhang/', '~/')
  p = string.gsub(p, 'src/main/java/com/', 'MAIN/')
  p = string.gsub(p, 'src/test/java/com/', 'TEST/')
  p = string.gsub(p, 'NUCEF1A%-', '')
  path_display_cache[path] = p
  return p
end

return M
