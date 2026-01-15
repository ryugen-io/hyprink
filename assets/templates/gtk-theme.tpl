[manifest]
name = "gtk-theme"
version = "0.2.0"
authors = ["Kitchn"]
description = "Modern GTK3/GTK4 color theme using CSS custom properties"
license = "MIT"

# GTK3 CSS (legacy @define-color for compatibility)
[[targets]]
target = "~/.config/gtk-3.0/gtk.css"
content = """
/* Kitchn GTK3 - Sweet Dracula */
@define-color accent_bg_color {{ colors.primary }};
@define-color accent_fg_color {{ colors.bg }};
@define-color accent_color {{ colors.bright_blue }};
@define-color destructive_bg_color {{ colors.error }};
@define-color destructive_fg_color {{ colors.fg }};
@define-color success_bg_color {{ colors.success }};
@define-color success_fg_color {{ colors.bg }};
@define-color warning_bg_color {{ colors.warn }};
@define-color warning_fg_color {{ colors.bg }};
@define-color error_bg_color {{ colors.error }};
@define-color error_fg_color {{ colors.fg }};
@define-color window_bg_color {{ colors.bg }};
@define-color window_fg_color {{ colors.fg }};
@define-color view_bg_color {{ colors.bg }};
@define-color view_fg_color {{ colors.fg }};
@define-color headerbar_bg_color {{ colors.bg }};
@define-color headerbar_fg_color {{ colors.fg }};
@define-color card_bg_color {{ colors.black }};
@define-color card_fg_color {{ colors.fg }};
@define-color popover_bg_color {{ colors.bg }};
@define-color popover_fg_color {{ colors.fg }};
@define-color shade_color alpha(black, 0.25);
@define-color borders {{ colors.black }};

/* Base */
window, .background { background: @window_bg_color; color: @window_fg_color; }
*:disabled { opacity: 0.5; }
*:selected { background: {{ colors.selection_bg }}; color: {{ colors.selection_fg }}; }

/* Header */
headerbar { background: @headerbar_bg_color; border-bottom: 1px solid @borders; min-height: 38px; }
headerbar:backdrop { color: {{ colors.bright_black }}; }
headerbar .title { font-weight: 600; }
headerbar .subtitle { color: {{ colors.bright_black }}; font-size: 0.9em; }

/* Buttons */
button { background: @card_bg_color; border: 1px solid @borders; border-radius: 8px; padding: 8px 14px; }
button:hover { background: {{ colors.bright_black }}; border-color: @accent_bg_color; }
button:active, button:checked { background: @accent_bg_color; color: @accent_fg_color; }
button.suggested-action { background: @accent_bg_color; color: @accent_fg_color; border-color: @accent_bg_color; }
button.destructive-action { background: @destructive_bg_color; color: @destructive_fg_color; }
button.flat { background: transparent; border: none; }
button.flat:hover { background: alpha(@accent_bg_color, 0.15); }
button.circular { border-radius: 50%; min-width: 36px; min-height: 36px; padding: 0; }
button.pill { border-radius: 999px; padding: 8px 20px; }

/* Entries */
entry { background: @view_bg_color; border: 1px solid @borders; border-radius: 8px; padding: 8px 12px; caret-color: {{ colors.cursor }}; }
entry:focus { border-color: @accent_bg_color; box-shadow: 0 0 0 3px alpha(@accent_bg_color, 0.2); }
entry selection { background: {{ colors.selection_bg }}; }
textview, textview text { background: @view_bg_color; }

/* Lists */
list, listview, treeview { background: @view_bg_color; }
row { padding: 8px 10px; border-radius: 6px; margin: 2px 4px; }
row:selected { background: {{ colors.selection_bg }}; }
row:hover:not(:selected) { background: alpha(@accent_bg_color, 0.08); }

/* Scrollbar */
scrollbar { background: transparent; }
scrollbar slider { background: {{ colors.bright_black }}; border-radius: 99px; min-width: 8px; min-height: 8px; margin: 4px; }
scrollbar slider:hover { background: @accent_bg_color; }
scrollbar slider:active { background: @accent_color; }

/* Controls */
check, radio { background: @view_bg_color; border: 2px solid {{ colors.bright_black }}; min-width: 20px; min-height: 20px; }
check { border-radius: 6px; }
radio { border-radius: 50%; }
check:checked, radio:checked { background: @accent_bg_color; border-color: @accent_bg_color; }
check:hover, radio:hover { border-color: @accent_bg_color; }
switch { background: @card_bg_color; border-radius: 99px; min-width: 50px; min-height: 26px; }
switch:checked { background: @accent_bg_color; }
switch slider { background: @window_fg_color; border-radius: 50%; min-width: 22px; min-height: 22px; margin: 2px; }

/* Progress/Scale */
progressbar, scale trough { background: @card_bg_color; border-radius: 99px; }
progressbar progress, scale highlight { background: @accent_bg_color; border-radius: 99px; }
progressbar.horizontal trough, progressbar.horizontal progress { min-height: 8px; }
scale slider { background: @window_fg_color; border-radius: 50%; min-width: 18px; min-height: 18px; box-shadow: 0 1px 3px alpha(black, 0.3); }
scale slider:hover { background: @accent_bg_color; }

/* Popover/Menu */
menu, .menu, popover { background: @popover_bg_color; border: 1px solid @borders; border-radius: 12px; box-shadow: 0 4px 16px alpha(black, 0.3); padding: 6px; }
menuitem { padding: 8px 14px; border-radius: 8px; }
menuitem:hover { background: {{ colors.selection_bg }}; }
tooltip { background: @card_bg_color; border: 1px solid @borders; border-radius: 8px; padding: 6px 10px; }

/* Tabs */
notebook > header { background: @window_bg_color; border-bottom: 1px solid @borders; }
notebook > header > tabs > tab { background: transparent; border-bottom: 3px solid transparent; color: {{ colors.bright_black }}; padding: 10px 18px; }
notebook > header > tabs > tab:checked { border-bottom-color: @accent_bg_color; color: @window_fg_color; }
notebook > header > tabs > tab:hover:not(:checked) { color: @window_fg_color; background: alpha(@accent_bg_color, 0.05); }

/* Sidebar */
.sidebar { background: @window_bg_color; border-right: 1px solid @borders; }
.sidebar row { padding: 10px 14px; }
.sidebar row:selected { background: {{ colors.selection_bg }}; }

/* Cards */
.card, frame { background: @card_bg_color; border-radius: 12px; padding: 12px; }

/* Status */
.info, infobar.info { background: alpha({{ colors.info }}, 0.15); color: {{ colors.info }}; }
.warning, infobar.warning { background: alpha(@warning_bg_color, 0.15); color: @warning_bg_color; }
.error, infobar.error { background: alpha(@error_bg_color, 0.15); color: @error_bg_color; }
.success, infobar.success { background: alpha(@success_bg_color, 0.15); color: @success_bg_color; }
infobar { border-radius: 8px; padding: 8px 12px; }

/* Misc */
separator { background: @borders; min-height: 1px; min-width: 1px; }
*:link { color: {{ colors.info }}; }
*:link:hover { color: @accent_bg_color; }
calendar { background: @view_bg_color; border-radius: 12px; }
calendar:selected { background: @accent_bg_color; color: @accent_fg_color; border-radius: 50%; }
spinbutton { background: @view_bg_color; border: 1px solid @borders; border-radius: 8px; }
spinbutton button { background: @card_bg_color; border: none; }
"""

# GTK4 CSS (modern CSS custom properties)
[[targets]]
target = "~/.config/gtk-4.0/gtk.css"
content = """
/* Kitchn GTK4 - Sweet Dracula (libadwaita compatible) */
:root {
  /* Accent */
  --accent-bg-color: {{ colors.primary }};
  --accent-fg-color: {{ colors.bg }};
  --accent-color: {{ colors.bright_blue }};

  /* Destructive */
  --destructive-bg-color: {{ colors.error }};
  --destructive-fg-color: {{ colors.fg }};
  --destructive-color: {{ colors.bright_red }};

  /* Success/Warning/Error */
  --success-bg-color: {{ colors.success }};
  --success-fg-color: {{ colors.bg }};
  --success-color: {{ colors.bright_green }};
  --warning-bg-color: {{ colors.warn }};
  --warning-fg-color: {{ colors.bg }};
  --warning-color: {{ colors.bright_yellow }};
  --error-bg-color: {{ colors.error }};
  --error-fg-color: {{ colors.fg }};
  --error-color: {{ colors.bright_red }};

  /* Window */
  --window-bg-color: {{ colors.bg }};
  --window-fg-color: {{ colors.fg }};

  /* View */
  --view-bg-color: {{ colors.bg }};
  --view-fg-color: {{ colors.fg }};

  /* Header */
  --headerbar-bg-color: {{ colors.bg }};
  --headerbar-fg-color: {{ colors.fg }};
  --headerbar-border-color: {{ colors.black }};
  --headerbar-backdrop-color: {{ colors.bg }};

  /* Card */
  --card-bg-color: {{ colors.black }};
  --card-fg-color: {{ colors.fg }};

  /* Dialog */
  --dialog-bg-color: {{ colors.bg }};
  --dialog-fg-color: {{ colors.fg }};

  /* Popover */
  --popover-bg-color: {{ colors.bg }};
  --popover-fg-color: {{ colors.fg }};

  /* Sidebar */
  --sidebar-bg-color: {{ colors.bg }};
  --sidebar-fg-color: {{ colors.fg }};
  --sidebar-border-color: {{ colors.black }};

  /* Misc */
  --shade-color: rgba(0, 0, 0, 0.25);
  --scrollbar-outline-color: transparent;
}

/* Base */
window, .background { background: var(--window-bg-color); color: var(--window-fg-color); }
*:disabled { opacity: 0.5; }
selection { background: {{ colors.selection_bg }}; color: {{ colors.selection_fg }}; }

/* Header */
headerbar { background: var(--headerbar-bg-color); border-bottom: 1px solid var(--headerbar-border-color); min-height: 38px; }
headerbar:backdrop { color: {{ colors.bright_black }}; }
headerbar .title { font-weight: 600; }
headerbar .subtitle { color: {{ colors.bright_black }}; font-size: 0.9em; }
windowcontrols button { background: transparent; border: none; min-width: 28px; min-height: 28px; border-radius: 50%; }
windowcontrols button:hover { background: var(--card-bg-color); }
windowcontrols button.close:hover { background: var(--destructive-bg-color); color: var(--destructive-fg-color); }

/* Buttons */
button { background: var(--card-bg-color); border: 1px solid var(--card-bg-color); border-radius: 8px; padding: 8px 14px; transition: all 150ms ease; }
button:hover { background: {{ colors.bright_black }}; border-color: var(--accent-bg-color); }
button:active, button:checked { background: var(--accent-bg-color); color: var(--accent-fg-color); }
button.suggested-action { background: var(--accent-bg-color); color: var(--accent-fg-color); }
button.suggested-action:hover { background: {{ colors.bright_blue }}; }
button.destructive-action { background: var(--destructive-bg-color); color: var(--destructive-fg-color); }
button.destructive-action:hover { background: var(--destructive-color); }
button.flat { background: transparent; border: none; }
button.flat:hover { background: color-mix(in srgb, var(--accent-bg-color) 15%, transparent); }
button.circular { border-radius: 50%; min-width: 36px; min-height: 36px; padding: 0; }
button.pill { border-radius: 999px; padding: 8px 20px; }
button.opaque { background: var(--accent-bg-color); color: var(--accent-fg-color); }

/* Entries */
entry { background: var(--view-bg-color); border: 1px solid var(--card-bg-color); border-radius: 8px; padding: 8px 12px; caret-color: {{ colors.cursor }}; }
entry:focus { border-color: var(--accent-bg-color); outline: 3px solid color-mix(in srgb, var(--accent-bg-color) 20%, transparent); outline-offset: -1px; }
entry > selection { background: {{ colors.selection_bg }}; }
textview, textview text { background: var(--view-bg-color); }

/* Lists */
list, listview, columnview { background: var(--view-bg-color); }
listview > row, list > row { padding: 8px 10px; border-radius: 6px; margin: 2px 4px; }
listview > row:selected, list > row:selected { background: {{ colors.selection_bg }}; }
listview > row:hover:not(:selected), list > row:hover:not(:selected) { background: color-mix(in srgb, var(--accent-bg-color) 8%, transparent); }

/* Scrollbar */
scrollbar { background: transparent; }
scrollbar slider { background: {{ colors.bright_black }}; border-radius: 99px; min-width: 8px; min-height: 8px; margin: 4px; }
scrollbar slider:hover { background: var(--accent-bg-color); }
scrollbar slider:active { background: var(--accent-color); }

/* Controls */
check, radio { background: var(--view-bg-color); border: 2px solid {{ colors.bright_black }}; min-width: 20px; min-height: 20px; transition: all 150ms ease; }
check { border-radius: 6px; }
radio { border-radius: 50%; }
check:checked, radio:checked { background: var(--accent-bg-color); border-color: var(--accent-bg-color); }
check:hover, radio:hover { border-color: var(--accent-bg-color); }
switch { background: var(--card-bg-color); border-radius: 99px; min-width: 50px; min-height: 26px; }
switch:checked { background: var(--accent-bg-color); }
switch > image { background: var(--window-fg-color); border-radius: 50%; min-width: 22px; min-height: 22px; margin: 2px; }

/* Progress/Scale */
progressbar > trough { background: var(--card-bg-color); border-radius: 99px; min-height: 8px; }
progressbar > trough > progress { background: var(--accent-bg-color); border-radius: 99px; }
scale > trough { background: var(--card-bg-color); border-radius: 99px; min-height: 8px; }
scale > trough > highlight { background: var(--accent-bg-color); border-radius: 99px; }
scale > trough > slider { background: var(--window-fg-color); border-radius: 50%; min-width: 18px; min-height: 18px; box-shadow: 0 1px 3px rgba(0,0,0,0.3); }
scale > trough > slider:hover { background: var(--accent-bg-color); }

/* Popover/Menu */
popover, popover.menu { background: var(--popover-bg-color); border: 1px solid var(--card-bg-color); border-radius: 12px; box-shadow: 0 4px 16px rgba(0,0,0,0.3); padding: 6px; }
popover.menu modelbutton { padding: 8px 14px; border-radius: 8px; }
popover.menu modelbutton:hover { background: {{ colors.selection_bg }}; }
tooltip { background: var(--card-bg-color); border: 1px solid {{ colors.bright_black }}; border-radius: 8px; }

/* Tabs */
notebook > header { background: var(--window-bg-color); border-bottom: 1px solid var(--card-bg-color); }
notebook > header > tabs > tab { background: transparent; border-bottom: 3px solid transparent; color: {{ colors.bright_black }}; padding: 10px 18px; }
notebook > header > tabs > tab:checked { border-bottom-color: var(--accent-bg-color); color: var(--window-fg-color); }
notebook > header > tabs > tab:hover:not(:checked) { color: var(--window-fg-color); background: color-mix(in srgb, var(--accent-bg-color) 5%, transparent); }
tabbar { background: var(--window-bg-color); border-bottom: 1px solid var(--card-bg-color); }
tabbar tab { background: transparent; border-bottom: 3px solid transparent; color: {{ colors.bright_black }}; }
tabbar tab:selected { border-bottom-color: var(--accent-bg-color); color: var(--window-fg-color); }

/* Sidebar */
.navigation-sidebar { background: var(--sidebar-bg-color); border-right: 1px solid var(--sidebar-border-color); }
.navigation-sidebar row { padding: 10px 14px; border-radius: 8px; margin: 2px 6px; }
.navigation-sidebar row:selected { background: {{ colors.selection_bg }}; }

/* Status */
.info { background: color-mix(in srgb, {{ colors.info }} 15%, transparent); color: {{ colors.info }}; }
.warning { background: color-mix(in srgb, var(--warning-bg-color) 15%, transparent); color: var(--warning-bg-color); }
.error { background: color-mix(in srgb, var(--error-bg-color) 15%, transparent); color: var(--error-bg-color); }
.success { background: color-mix(in srgb, var(--success-bg-color) 15%, transparent); color: var(--success-bg-color); }

/* Cards (libadwaita) */
.card { background: var(--card-bg-color); border-radius: 12px; padding: 12px; }
preferencesgroup > box > box { background: var(--card-bg-color); border-radius: 12px; }
actionrow { padding: 12px; }
actionrow:hover { background: color-mix(in srgb, var(--accent-bg-color) 5%, transparent); }

/* Dialog */
messagedialog, dialog { background: var(--dialog-bg-color); color: var(--dialog-fg-color); }

/* Toast */
toast { background: var(--card-bg-color); border-radius: 999px; padding: 8px 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }

/* View Switcher */
viewswitcher button { background: transparent; }
viewswitcher button:checked { background: {{ colors.selection_bg }}; }

/* Search */
searchbar > revealer > box { background: var(--window-bg-color); border-bottom: 1px solid var(--card-bg-color); }

/* Dropdown */
dropdown > button { background: var(--card-bg-color); border: 1px solid var(--card-bg-color); border-radius: 8px; }
dropdown > button:hover { border-color: var(--accent-bg-color); }

/* Misc */
separator { background: var(--card-bg-color); min-height: 1px; min-width: 1px; }
link, *:link { color: {{ colors.info }}; }
link:hover, *:link:hover { color: var(--accent-bg-color); }
calendar { background: var(--view-bg-color); border-radius: 12px; }
calendar:selected { background: var(--accent-bg-color); color: var(--accent-fg-color); border-radius: 50%; }
spinbutton { background: var(--view-bg-color); border: 1px solid var(--card-bg-color); border-radius: 8px; }
spinbutton > button { background: var(--card-bg-color); border: none; }
*:focus-visible { outline: 2px solid color-mix(in srgb, var(--accent-bg-color) 50%, transparent); outline-offset: 2px; }
levelbar > trough { background: var(--card-bg-color); border-radius: 99px; }
levelbar > trough > block.filled { background: var(--accent-bg-color); border-radius: 99px; }
levelbar > trough > block.low { background: var(--error-bg-color); }
levelbar > trough > block.high { background: var(--success-bg-color); }
"""

[hooks]
