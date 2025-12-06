[meta]
id = "hypr.fish"

[[templates]]
target = "~/.config/fish/conf.d/hypr_theme.fish"
content = """
set -gx HYPR_PRIMARY "{{ colors.primary }}"
set -gx HYPR_FONT_MONO "{{ fonts.mono }}"
set -gx HYPR_ICON_ERR "{{ icons.error }}"

function hlog
    corelog $argv
end
"""
