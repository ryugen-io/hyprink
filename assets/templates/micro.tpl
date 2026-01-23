[manifest]
id = "micro"
name = "micro"
version = "1.0.0"
description = "Micro editor colorscheme with truecolor support"
authors = ["Hyprink"]

[[targets]]
target = "~/.config/micro/colorschemes/hyprink.micro"
content = '''
# Hyprink Theme for Micro Editor (Truecolor)

color-link default "{{ colors.fg }},#13172a"
color-link comment "{{ colors.bright_black }}"

color-link identifier "{{ colors.success }}"
color-link identifier.class "{{ colors.info }}"
color-link identifier.var "{{ colors.fg }}"

color-link constant "{{ colors.primary }}"
color-link constant.number "{{ colors.fg }}"
color-link constant.string "{{ colors.bright_yellow }}"

color-link symbol "{{ colors.secondary }}"
color-link symbol.brackets "{{ colors.fg }}"
color-link symbol.tag "{{ colors.primary }}"

color-link type "italic {{ colors.info }}"
color-link type.keyword "{{ colors.secondary }}"

color-link special "{{ colors.secondary }}"
color-link statement "{{ colors.secondary }}"
color-link preproc "{{ colors.secondary }}"

color-link underlined "{{ colors.secondary }}"
color-link error "bold {{ colors.error }}"
color-link todo "bold {{ colors.secondary }}"

color-link hlsearch "{{ colors.bg }},{{ colors.success }}"

color-link diff-added "{{ colors.success }}"
color-link diff-modified "{{ colors.warn }}"
color-link diff-deleted "{{ colors.error }}"

color-link gutter-error "{{ colors.error }}"
color-link gutter-warning "{{ colors.warn }}"

color-link statusline "{{ colors.bg }},{{ colors.fg }}"
color-link statusline.inactive "{{ colors.bright_black }},{{ colors.black }}"
color-link tabbar "{{ colors.bg }},{{ colors.fg }}"
color-link tabbar.active "{{ colors.bg }},{{ colors.secondary }}"
color-link indent-char "{{ colors.bright_black }}"
color-link line-number "{{ colors.bright_black }}"
color-link current-line-number "{{ colors.fg }}"

color-link cursor-line "{{ colors.selection_bg }},{{ colors.fg }}"
color-link color-column "{{ colors.selection_bg }}"
color-link type.extended "default"

color-link match-brace "{{ colors.bg }},{{ colors.primary }}"

color-link tab-error "{{ colors.error }}"
color-link trailingws "{{ colors.error }}"
'''

[[targets]]
target = "~/.config/micro/settings.json"
content = '''
{
    "autoclose": true,
    "autoindent": true,
    "autosave": 0,
    "colorcolumn": 100,
    "colorscheme": "hyprink",
    "cursorline": true,
    "diffgutter": true,
    "encoding": "utf-8",
    "eofnewline": true,
    "fastdirty": false,
    "ignorecase": true,
    "indentchar": " ",
    "infobar": true,
    "keepautoindent": false,
    "mouse": true,
    "mkparents": true,
    "pluginchannels": [
        "https://raw.githubusercontent.com/micro-editor/plugin-channel/master/channel.json"
    ],
    "rmtrailingws": true,
    "ruler": true,
    "savecursor": true,
    "savehistory": true,
    "saveundo": true,
    "scrollbar": true,
    "scrollmargin": 3,
    "scrollspeed": 2,
    "smartpaste": true,
    "softwrap": false,
    "splitbottom": true,
    "splitright": true,
    "statusformatl": "$(filename) $(modified)($(line),$(col)) $(status.paste)| ft:$(opt:filetype) | $(opt:fileformat) | $(opt:encoding)",
    "statusformatr": "$(bind:ToggleKeyMenu): bindings, $(bind:ToggleHelp): help",
    "statusline": true,
    "syntax": true,
    "tabmovement": false,
    "tabsize": 4,
    "tabstospaces": true,
    "useprimary": true
}
'''

[hooks]
info = "echo '[OK] Micro theme applied. Run: micro -plugin install filemanager fzf jump lsp'"
