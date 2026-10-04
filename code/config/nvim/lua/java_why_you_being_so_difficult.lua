local path = require('java-core.utils.path')
local lsp_utils = require('java-core.utils.lsp')

local M = {}

local settings_xml = '/Users/nvkhang/dots/mvn/settings.xml'

local java_configuration = function(active)
  if not active then return {} end
  return {
    maven = {
      globalSettings = settings_xml,
      userSettings = settings_xml,
    },
  }
end

M.settings = function(is_offline)
  return {
    java = {
      configuration = java_configuration(true),
      eclipse = { downloadSources = true },
      import = {
        gradle = { enabled = false },
        maven = {
          enabled = true,
          offline = { enabled = is_offline },
        },
      },
      maven = { downloadSources = true },
      runtimes = {
        {
          name = 'JavaSE-17',
          path = '/Library/Java/JavaVirtualMachines/liberica-jdk-17.jdk/Contents/Home',
        },
      },
    },
  }
end

M.setup = function(opts)
  local is_offline = opts.offline
  if is_offline == nil then is_offline = true end

  -- https://github.com/nvim-java/nvim-java
  local nvim_java = require('java')
  nvim_java.setup {
    checks = {
      nvim_version = true, -- Check Neovim version
      nvim_jdtls_conflict = true, -- Check for nvim-jdtls conflict
    },
    jdtls = { path = '/Users/nvkhang/.local/k-jdtls/jdtls-1.60.0' },
    lombok = { path = '/Users/nvkhang/.local/k-jdtls/lombok-1.18.46.jar' },
    java_test = { enable = false },
    java_debug_adapter = { enable = false },
    spring_boot_tools = { enable = true },
    jdk = { path = '/Users/nvkhang/.local/k-jdtls/jdk-25.0.3.jdk' },
  }
  opts = vim.lsp.config['jdtls']
  require('brew.lsp').add_on_attach(opts)

  opts.settings = M.settings(is_offline)
  opts.filetypes = { 'java', 'xml' }
  opts.root_dir = vim.fn.getcwd()
  opts.cmd_env['ART_USER'] = vim.env.ART_USER
  opts.cmd_env['ART_PASS'] = vim.env.ART_PASS
  vim.lsp.config('jdtls', opts)
end

return M
