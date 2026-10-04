local M = {}

local default_on_attach = function(_, bufnr)
  -- Disable LSP-based syntax highlighting. This introduces a color change
  -- after LSP gets attached.
  for _, group in ipairs(vim.fn.getcompletion('@lsp', 'highlight')) do
    vim.api.nvim_set_hl(0, group, {})
  end
  local x = { buffer = bufnr, noremap = true }
  vim.keymap.set('n', 'gd', vim.lsp.buf.definition, x)
  vim.keymap.set('n', 'gD', vim.lsp.buf.declaration, x)
  vim.keymap.set('n', 'gt', vim.lsp.buf.type_definition, x)
  vim.keymap.set('n', 'gr', vim.lsp.buf.references, x)
  vim.keymap.set('n', 'gi', vim.lsp.buf.implementation, x)
  vim.keymap.set('n', 'K', vim.lsp.buf.hover, x)
end

M.add_on_attach = function(opts)
  if opts.on_attach ~= nil then
    local user_on_attach = opts.on_attach
    opts.on_attach = function(client, bufnr)
      default_on_attach(client, bufnr)
      user_on_attach(client, bufnr)
    end
  else
    opts.on_attach = default_on_attach
  end
end

M.add = setmetatable({}, {
  __newindex = function(_, key, opts)
    M.add_on_attach(opts)
    vim.lsp.config(key, opts)
    vim.lsp.enable(key)
  end,
})

return M
