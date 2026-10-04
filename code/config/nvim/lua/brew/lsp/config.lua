-- vim:fmr=--<,-->
local lsp = require('brew.lsp')

-- vim.lsp.log.set_level('trace')

--< Rust
lsp.add['rust_analyzer'] = {
  filetypes = { 'rust' },
  cmd = { 'rust-analyzer' },
  root_markers = { 'Cargo.toml', 'rust-project.json' },
  settings = {
    ['rust-analyzer'] = {
      cargo = {
        allTargets = true,
        features = 'all',
      },
      check = {
        allTargets = true,
      },
      imports = { granularity = { group = 'item' } },

      -- procMacro = {
      --   ignored = {
      --     -- Ignoring this fixed LSP go-to-definition within `#[tokio::test]`.
      --     -- https://github.com/rust-lang/rust-analyzer/issues/12362
      --     -- ['tokio-macros'] = { 'test' },
      --   },
      -- },
    },
  },
} -->
--< GoLang
lsp.add['gopls'] = {
  filetypes = { 'go' },
  root_markers = { 'go.mod', '.git' },
} -->
--< Python
lsp.add['python'] = {
  filetypes = { 'python' },
  cmd = { 'pyright-langserver', '--stdio' },
  root_markers = { '.git' },
  settings = {
    python = {
      analysis = {
        useLibraryCodeForTypes = true,
      },
    },
  },
} -->
--< Lua LS (lua-language-server)
lsp.add['lua_ls'] = {
  filetypes = { 'lua' },
  cmd = { 'lua-language-server' },
  settings = {
    Lua = {
      runtime = { version = 'LuaJIT' },
      diagnostics = {
        globals = { 'vim', 'awesome', 'client', 'root', 'screen' },
      },
      workspace = {
        checkThirdParty = false,
        library = {
          vim.env.VIMRUNTIME,
          '/usr/share/awesome/lib/',
        },
      },
      telemetry = { enable = false },
    },
  },
} -->
--< Clang
lsp.add['clang'] = {
  filetypes = { 'c', 'cpp' },
  cmd = { 'clangd' },
} -->
--< Zig LS
lsp.add['zls'] = {
  filetypes = { 'zig', 'zir' },
  cmd = { 'zls' },
  root_markers = { 'zls.json', 'build.zig', '.git' },
  on_attach = function(_, bufnr)
    local x = { buffer = bufnr, noremap = true }
    vim.keymap.set('n', 'gD', vim.lsp.buf.definition, x)
    vim.keymap.set('n', 'gd', vim.lsp.buf.declaration, x)
  end,
} -->
--< TypeScript LS
lsp.add['ts_ls'] = {
  filetypes = {
    'javascriptreact',
    'typescriptreact',
    'javascript',
    'typescript',
  },
  cmd = { 'typescript-language-server', '--stdio' },
  root_markers = { 'tsconfig.json', 'jsconfig.json', 'package.json', '.git' },
} -->
--< Cucumber
lsp.add['cucumber_language_server'] = {
  filetypes = { 'cucumber' },
  root_dir = vim.fn.getcwd(),
  cmd = { 'cucumber-language-server', '--stdio' },
  settings = {
    cucumber = {
      glue = {
        '**/src/test/java/**/*.java',
        '*/src/test/java/**/*.java',
        'src/test/java/**/*.java',
      },
    },
  },
} -->
--< Rust with Bazel
local rust_with_bazel = false

if not rust_with_bazel then
  lsp.add['rust_analyzer'] = {
    filetypes = { 'rust' },
    cmd = { 'rust-analyzer' },
    root_markers = { 'Cargo.toml', 'rust-project.json' },
    settings = {
      ['rust-analyzer'] = {
        cargo = {
          allTargets = true,
          features = 'all',
        },
        check = {
          allTargets = true,
        },
        imports = { granularity = { group = 'item' } },
        procMacro = {
          ignored = {
            -- Ignoring this fixed LSP go-to-definition within `#[tokio::test]`.
            -- https://github.com/rust-lang/rust-analyzer/issues/12362
            -- ['tokio-macros'] = { 'test' },
          },
        },
      },
    },
  }
else
  lsp.add['rust_analyzer'] = {
    filetypes = { 'rust' },
    root_dir = '/home/khang/mono/code/experiments/bazel',
    -- root_markers = { 'MODULE.bazel', 'MODULE.bazel.lock' },
    --------------------------------------------------

    cmd = {
      '/home/khang/.cache/bazel/_bazel_khang/cache/repos/v1/contents/148ea448c7b03c6257f3bb3eb7ed04b8b85b20fd7d460b1b3c136d34dd21c869/37007994-ea4e-48fa-b85f-232987625fac/bin/rust-analyzer',
    },
    settings = {
      ['rust-analyzer'] = {
        workspace = {
          discoverConfig = {
            command = {
              '/home/khang/mono/code/experiments/bazel/.rules_rust_analyzer/discover_bazel_rust_project.exe',
              '{arg}',
            },
            progressLabel = 'rules_rust',
            filesToWatch = {
              'BUILD',
              'BUILD.bazel',
              'MODULE.bazel',
              'WORKSPACE',
              'WORKSPACE.bazel',
            },
          },
        },
        procMacro = {
          server = '/home/khang/.cache/bazel/_bazel_khang/cache/repos/v1/contents/148ea448c7b03c6257f3bb3eb7ed04b8b85b20fd7d460b1b3c136d34dd21c869/37007994-ea4e-48fa-b85f-232987625fac/libexec/rust-analyzer-proc-macro-srv',
        },
        rustfmt = {
          overrideCommand = {
            '/home/khang/.cache/bazel/_bazel_khang/cache/repos/v1/contents/3f5ffa689a3cb2162b1a62be1071223d13e89a5dbb1f1d418b0ace80edef155c/ac2a578c-f5d7-42bb-bdc2-a9f0a070396f/bin/rustfmt',
          },
        },
        lens = { enable = true },
      },
    },

    --------------------------------------------------
  }
end -->
