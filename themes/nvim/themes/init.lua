local catppuccin = require 'themes.catppuccin'
local dracula = require 'themes.dracula'
local everblush = require 'themes.everblush'
local everforest = require 'themes.everforest'
local kanagawa = require 'themes.kanagawa'
local nord = require 'themes.nord'
local rose_pine = require 'themes.rose_pine'
local tokyonight = require 'themes.tokyonight'
local gruvbox = require 'themes.gruvbox'

-- Load the selected theme
local M = {}
M.color = 'gruvbox' -- just change this to switch themes

function M.apply()
  local ok, _ = pcall(vim.cmd.colorscheme, M.color)
  if not ok then
    vim.notify("Colorscheme '" .. M.color .. "' not found!", vim.log.levels.ERROR)
  end
end

local loader = {
  'nvim',
  priority = 999, -- run after theme plugins (priority 1000)
  lazy = false,
  config = function()
    M.apply()
  end,
}

return {
  catppuccin,
  dracula,
  everblush,
  everforest,
  kanagawa,
  nord,
  rose_pine,
  tokyonight,
  gruvbox,
  loader,
}

