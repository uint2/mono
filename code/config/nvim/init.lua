-- Bootstrap `lazy.nvim` by Folke.
require('brew.lazy-bootstrap')

vim.lsp.log.set_level(vim.log.levels.OFF)

local _ENABLE_JAVA_LSP = vim.env.LSP ~= '0'

vim.diagnostic.config { underline = false, virtual_text = true }

--[[ Plugin Archive
    * nvim-treesitter/playground
    * wuelnerdotexe/vim-astro
    * m4xshen/autoclose.nvim
--]]

require('lazy').setup {
  rocks = { enabled = false },
  spec = {
    'tpope/vim-surround',
    'vimplug/nvim-colorizer.lua',
    {
      'nvim-java/nvim-java',
      enabled = _ENABLE_JAVA_LSP,
      dependencies = {
        'MunifTanjim/nui.nvim',
        'mfussenegger/nvim-dap',
        'JavaHello/spring-boot.nvim',
      },
      dev = true,
      dir = vim.fn.stdpath('config') .. '/nvim-java',
      config = function()
        local jwybsd = require('java_why_you_being_so_difficult').setup {
          offline = true,
        }
        vim.lsp.enable('jdtls')
      end,
    },
    --< nvim-treesitter/nvim-treesitter
    {
      'nvim-treesitter/nvim-treesitter',
      commit = '90cd6580e720caedacb91fdd587b747a6e77d61f', -- the last one before v11 gets dropped.
      lazy = false,
      build = ':TSUpdate',
    }, -->
    --< nvim-treesitter/nvim-treesitter-context
    {
      'nvim-treesitter/nvim-treesitter-context',
      enabled = false,
      opts = {
        enable = true, -- Enable this plugin (Can be enabled/disabled later via commands)
        multiwindow = false, -- Enable multiwindow support.
        max_lines = 0, -- How many lines the window should span. Values <= 0 mean no limit.
        min_window_height = 0, -- Minimum editor window height to enable context. Values <= 0 mean no limit.
        line_numbers = true,
        multiline_threshold = 20, -- Maximum number of lines to show for a single context
        trim_scope = 'outer', -- Which context lines to discard if `max_lines` is exceeded. Choices: 'inner', 'outer'
        mode = 'cursor', -- Line used to calculate context. Choices: 'cursor', 'topline'
        -- Separator between context and content. Should be a single character string, like '-'.
        -- When separator is set, the context will only show up when there are at least 2 lines above cursorline.
        separator = nil,
        zindex = 20, -- The Z-index of the context window
        on_attach = nil, -- (fun(buf: integer): boolean) return false to disable attaching
      },
    }, -->
    --< f-person/git-blame.nvim
    {
      'f-person/git-blame.nvim',
      opts = {
        message_template = '[<sha>] <author> <summary> (<date>)', -- template for the blame message, check the Message template section for more options
        date_format = '%Y%m%dT%H:%M',
        enabled = false,
      },
      keys = { { '<leader>gb', ':GitBlameToggle<cr>', { silent = true } } },
    }, -->
    --< nvim-telescope/telescope.nvim
    {
      'nvim-telescope/telescope.nvim',
      dependencies = {
        'nvim-telescope/telescope-fzy-native.nvim',
        'nvim-lua/plenary.nvim',
      },
      keys = function()
        local search = require('brew.telescope_search')
        local m = require('minimath.telescopes')
        return {
          { '<c-p>', search.files.repo },
          { '<c-f>', search.files.cwd },
          { '<leader>ps', search.string.repo },
          { '<leader>pS', search.string.repo_live },
          { '<leader>pf', search.string.cursor_cwd },
          { '<leader>pF', search.string.cursor_repo },
          { '<leader>pw', search.string.cwd },
          { '<leader>sd', search.files.dots },
          { '<leader>su', search.files.university },
          -- minimath repo searchers
          { '<leader>pm', function() m.theorem_search('j') end, ft = 'tex' },
          { '<leader>pt', function() m.theorem_search('y') end, ft = 'tex' },
          {
            '<leader>h',
            function() m.theorem_search('h') end,
            mode = 'v',
            ft = 'tex',
          },
          {
            '<leader>a',
            function() m.theorem_search('a') end,
            mode = 'v',
            ft = 'tex',
          },
          -- lean theorem searchers
          { '<leader>pm', m.lean_theorem_search, ft = 'lean' },
        }
      end,
      config = function()
        local telescope = require('telescope')
        local actions = require('telescope.actions')
        local a_state = require('telescope.actions.state')
        local a_set = require('telescope.actions.set')
        local from_entry = require('telescope.from_entry')
        local omega = require('omega')

        -- Load all results to quickfix list AND jump to the selected one.
        local qf_and_jump = function(bufnr)
          local p, qf = a_state.get_current_picker(bufnr), {}
          for e in p.manager:iter() do
            local i, t, v = { bufnr = e.bufnr }, e.text, e.value
            i.filename = from_entry.path(e, false, false)
            i.lnum, i.col = vim.F.if_nil(e.lnum, 1), vim.F.if_nil(e.col, 1)
            i.text = t and t or type(v) == 'table' and v.text or v
            table.insert(qf, i)
          end
          vim.fn.setqflist(qf, 'r')
          return a_set.select(bufnr, 'default')
        end

        telescope.setup {
          defaults = {
            layout_strategy = 'flex',
            layout_config = { width = 0.99, height = 0.9 },
            vimgrep_arguments = { 'rg', '--vimgrep', '--hidden' },
            color_devicons = false,
            selection_caret = '> ',
            mappings = { i = { ['<esc>'] = actions.close } },
            path_display = omega.path_display,
            preview = { treesitter = false },
          },
          pickers = {
            git_files = {
              hidden = true,
              previewer = false,
            },
            find_files = {
              hidden = true,
              previewer = false,
            },
            grep_string = {
              mappings = { i = { ['<CR>'] = qf_and_jump } },
            },
          },
          extensions = { fzy_native = {} },
        }

        telescope.load_extension('fzy_native')
      end,
    }, -->
    --< saghen/blink.cmp
    {
      'saghen/blink.cmp',
      version = '1.*',
      opts = {
        keymap = {
          ['<C-l>'] = { 'accept', 'select_and_accept', 'fallback' },
          ['<C-p>'] = { 'select_prev', 'fallback' },
          ['<C-n>'] = { 'select_next', 'fallback' },
        },
        completion = {
          menu = {
            draw = {
              columns = { { 'label' }, { 'kind' } },
              components = {
                label = {
                  width = { fill = true, max = 36 },
                  text = function(ctx)
                    if ctx.label_detail == '' then
                      return ctx.label
                    else
                      return ctx.label .. ' ' .. ctx.label_detail
                    end
                  end,
                },
              },
            },
          },
          ghost_text = { enabled = false },
          list = { selection = { preselect = false } },
          documentation = { auto_show = true },
        },
        sources = { default = { 'lsp', 'path', 'buffer' } },
        fuzzy = { implementation = 'prefer_rust' },
      },
    }, -->
    --< folke/noice.nvim
    {
      'folke/noice.nvim',
      enabled = true,
      event = 'VeryLazy',
      opts = {
        notify = { enabled = false },
        cmdline = { enabled = false },
        messages = { enabled = false },
        lsp = {
          override = {
            ['vim.lsp.util.convert_input_to_markdown_lines'] = true,
            ['vim.lsp.util.stylize_markdown'] = true,
          },
          message = {
            enabled = false,
            view = 'notify',
            opts = {},
          },
        },
        presets = { lsp_doc_border = false },
        routes = {
          {
            filter = {
              event = 'lsp',
              kind = 'progress',
              find = 'Validate documents',
            },
            opts = { skip = true },
          },
          {
            filter = {
              event = 'lsp',
              kind = 'progress',
              find = 'Publish Diagnostics',
            },
            opts = { skip = true },
          },
        },
      },
    }, -->
    --< stevearc/conform.nvim
    {
      'stevearc/conform.nvim',
      opts = {
        notify_on_error = true,
        formatters = {
          rustfmt = { options = { default_edition = '2024' } },
          shfmt = function() return { prepend_args = { '--indent', '2' } } end,
          ['google-java-format'] = {
            prepend_args = {
              '--skip-removing-unused-imports',
              '--skip-reflowing-long-strings',
            },
          },
        },
        async = true,
        formatters_by_ft = {
          astro = { 'prettier' },
          bash = { 'shfmt' },
          bzl = { 'buildifier' },
          c = { 'clang_format' },
          cpp = { 'clang_format' },
          css = { 'prettier' },
          go = { 'gofmt' },
          html = { 'prettier' },
          java = { 'google-java-format' },
          javascript = { 'prettier' },
          javascriptreact = { 'prettier' },
          json = { 'prettier' },
          lua = { 'stylua' },
          markdown = { 'prettier' },
          ocaml = { 'ocamlformat' },
          python = { 'black' },
          rust = { 'rustfmt' },
          sh = { 'shfmt' },
          swift = { 'swiftformat' },
          typescript = { 'prettier' },
          typescriptreact = { 'prettier' },
          yaml = { 'prettier' },
          zig = { 'zigfmt' },
          zsh = { 'shfmt' },
        },
      },
    }, -->
    --< terrortylor/nvim-comment
    {
      'terrortylor/nvim-comment',
      main = 'nvim_comment',
      keys = {
        { '<C-c>', ':CommentToggle<CR>' },
        { '<C-c>', ':CommentToggle<CR>', mode = 'v' },
      },
      opts = { create_mappings = false },
    }, -->
    --< nguyenvukhang/nvim-toggler
    {
      'nguyenvukhang/nvim-toggler',
      opts = {
        inverses = {
          ['₂'] = '₁',
          ['- [ ]'] = '- [x]',
          ['shift'] = 'unshift',
          ['exact'] = 'refine',
          ['next'] = 'prev',
          ['odd'] = 'even',
          ['forall'] = 'exists',
          ['row'] = 'column',
          ['positive'] = 'negative',
          ['horizontal'] = 'vertical',
          ['above'] = 'below',
          ['Above'] = 'Below',
          ['min'] = 'max',
          ['width'] = 'height',
          ['sin'] = 'cos',
          ['begin'] = 'end',
          ['True'] = 'False',
          ['TRUE'] = 'FALSE',
          ['upper'] = 'lower',
          ['cot'] = 'tan',
          ['sec'] = 'csc',
          ['good'] = 'bad',
          ['ON'] = 'OFF',
          ['Open'] = 'Closed',
          ['Yes'] = 'No',
          ['and'] = 'or',
          ['head'] = 'tail',
          ['open'] = 'closed',
        },
        autoselect_longest_match = true,
      },
    }, -->
    --< nvim-lua/plenary.nvim (+ harpoon)
    {
      'nvim-lua/plenary.nvim',
      config = function() require('harpoon').my_setup() end,
    }, -->
    --< ibhagwan/fzf-lua
    {
      'ibhagwan/fzf-lua',
      enabled = false,
      opts = {
        winopts = {
          preview = { vertical = 'up:45%', horizontal = 'right:50%' },
        },
        hls = {
          border = 'Comment',
          preview_border = 'Comment',
        },
        -- Specific picker options
        files = {
          winopts = { preview = { hidden = true } },
        },
      },
      keys = function()
        local bfzf = require('brew.fzf')
        local fzf = require('fzf-lua')
        return {
          -- file searches.
          { '<C-f>', bfzf.files },
          { '<C-p>', bfzf.git_files },
          {
            '<leader>sd',
            function()
              fzf.files {
                cwd = vim.env.DOTS,
                winopts = { title = 'Search dots' },
              }
            end,
          },
          -- word searches.
          { '<leader>pw', bfzf.grep },
          { '<leader>ps', bfzf.git_grep },
          { '<leader>pf', bfzf.grep_cword },
        }
      end,
      config = function(spec)
        local fzf = require('fzf-lua')
        local brew = require('brew.server.utils')
        local rg = require('minimath.fzf')

        fzf.setup(spec.opts)

        local keymap = function(mode, keymap, callback)
          vim.keymap.set(mode, keymap, function()
            rg:load_minimath()
            fzf.fzf_exec(rg.fzf_choices, { actions = { ['enter'] = callback } })
          end, { silent = true, buffer = true })
        end

        brew.autocmd {
          pattern = '*.lean',
          callback = function()
            vim.keymap.set('n', '<leader>pm', function()
              rg:load_lean()
              fzf.fzf_exec(rg.fzf_choices, {
                actions = {
                  ['enter'] = function(fzf_choices)
                    rg.jump(rg:get_target(fzf_choices[1]))
                  end,
                },
              })
            end, { silent = true, buffer = true })
          end,
        }

        brew.autocmd {
          pattern = '*.tex',
          callback = function()
            keymap('n', '<leader>pm', function(fzf_choices)
              -- Jump to a theorem.
              rg.jump(rg:get_target(fzf_choices[1]))
            end)
            keymap('n', '<leader>pt', function(fzf_choices)
              -- Copy the theorem's SHA to empty register.
              vim.fn.setreg('', rg.get_sha(fzf_choices[1]))
            end)
            keymap('v', '<leader>h', function(fzf_choices)
              -- Surround the selection with a \href{...}{<selection>}
              local sha = rg.get_sha(fzf_choices[1])
              vim.fn.feedkeys('gv"xc\\href{' .. sha .. '}{}"xP')
            end)
            keymap('v', '<leader>a', function(fzf_choices)
              -- Replace the selection with a \autoref{...}
              local sha = rg.get_sha(fzf_choices[1])
              vim.fn.feedkeys('gv"xc\\autoref{' .. sha .. '}')
            end)
          end,
        }
      end,
    }, -->
    --< brachiosauruses/lean.nvim
    {
      'brachiosauruses/lean.nvim', -- Julian
      dependencies = {
        'neovim/nvim-lspconfig',
        'nvim-lua/plenary.nvim',
      },
      lazy = false,
      opts = function()
        require('brew.lsp').add['leanls'] =
          { init_options = { editDelay = 100000 } }
        return {
          infoview = {
            autoopen = false,
            -- show_term_goals = false,
          },
          inlay_hint = { enabled = false },
          progress_bars = { enable = false },
          goal_markers = {
            accomplished = '',
            unsolved = '',
          },
        }
      end,
      keys = { { '<leader>u', ':LeanInfoviewToggle<cr>', { silent = true } } },
    }, -->
  },
}

require('brew.lsp.config')

-- independent of plugins, server-friendly
require('brew.server.sets')
require('brew.server.remaps')
require('brew.server.commands')
require('brew.server.statusline')
require('brew.server.autocmd')

require('autoclose').setup()

vim.cmd('colo gruvbox8_generated')
-- vim:fmr=--<,-->
