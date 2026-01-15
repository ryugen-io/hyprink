[manifest]
name = "yazi"
version = "0.1.0"
authors = ["Kitchn"]
description = "Yazi file manager configuration with theme support"
license = "MIT"

[[targets]]
target = "~/.config/yazi/yazi.toml"
content = '''
[mgr]
show_hidden = true

# ===== OPENER DEFINITIONS =====
[opener]

# Audio
audio = [
    { run = 'ffplay -nodisp -autoexit "$@"', block = true, desc = "Play with ffplay" },
]

# Video
video = [
    { run = 'ffplay "$@"', orphan = true, desc = "Play with ffplay" },
]

# Images
image = [
    { run = 'imv "$@"', orphan = true, desc = "Open with imv" },
]

# PDF
pdf = [
    { run = 'xdg-open "$@"', orphan = true, desc = "Open PDF" },
]

# Documents
document = [
    { run = 'xdg-open "$@"', orphan = true, desc = "Open document" },
]

# Text editor
edit = [
    { run = 'micro "$@"', block = true, desc = "Edit with micro" },
]

# Text viewer
view = [
    { run = 'bat --paging=always "$@"', block = true, desc = "View with bat" },
]

# Archive
extract = [
    { run = 'unar "$@"', desc = "Extract archive" },
]

# Python
python = [
    { run = 'python "$@"', block = true, desc = "Run with python" },
]

# Shell
shell = [
    { run = 'bash "$@"', block = true, desc = "Run with bash" },
]

# Fallback
open = [
    { run = 'xdg-open "$@"', orphan = true, desc = "Open" },
]

# ===== FILE TYPE RULES =====
[open]
rules = [
    # Audio
    { mime = "audio/*", use = "audio" },
    { name = "*.mp3", use = "audio" },
    { name = "*.flac", use = "audio" },
    { name = "*.wav", use = "audio" },
    { name = "*.ogg", use = "audio" },
    { name = "*.m4a", use = "audio" },
    { name = "*.aac", use = "audio" },
    { name = "*.opus", use = "audio" },

    # Video
    { mime = "video/*", use = "video" },
    { name = "*.mp4", use = "video" },
    { name = "*.mkv", use = "video" },
    { name = "*.avi", use = "video" },
    { name = "*.mov", use = "video" },
    { name = "*.webm", use = "video" },

    # Images
    { mime = "image/*", use = "image" },
    { name = "*.jpg", use = "image" },
    { name = "*.jpeg", use = "image" },
    { name = "*.png", use = "image" },
    { name = "*.gif", use = "image" },
    { name = "*.bmp", use = "image" },
    { name = "*.svg", use = "image" },
    { name = "*.webp", use = "image" },

    # PDF
    { mime = "application/pdf", use = "pdf" },
    { name = "*.pdf", use = "pdf" },

    # Documents
    { name = "*.doc", use = "document" },
    { name = "*.docx", use = "document" },
    { name = "*.xls", use = "document" },
    { name = "*.xlsx", use = "document" },
    { name = "*.ppt", use = "document" },
    { name = "*.pptx", use = "document" },

    # Text files
    { mime = "text/*", use = ["edit", "view"] },
    { name = "*.txt", use = ["edit", "view"] },
    { name = "*.md", use = ["edit", "view"] },
    { name = "*.log", use = ["edit", "view"] },

    # Code files
    { name = "*.c", use = ["edit", "view"] },
    { name = "*.cpp", use = ["edit", "view"] },
    { name = "*.rs", use = ["edit", "view"] },
    { name = "*.go", use = ["edit", "view"] },
    { name = "*.java", use = ["edit", "view"] },
    { name = "*.js", use = ["edit", "view"] },
    { name = "*.ts", use = ["edit", "view"] },
    { name = "*.jsx", use = ["edit", "view"] },
    { name = "*.tsx", use = ["edit", "view"] },
    { name = "*.vue", use = ["edit", "view"] },
    { name = "*.php", use = ["edit", "view"] },
    { name = "*.rb", use = ["edit", "view"] },

    # Config files
    { name = "*.json", use = ["edit", "view"] },
    { name = "*.yaml", use = ["edit", "view"] },
    { name = "*.yml", use = ["edit", "view"] },
    { name = "*.toml", use = ["edit", "view"] },
    { name = "*.xml", use = ["edit", "view"] },
    { name = "*.ini", use = ["edit", "view"] },
    { name = "*.conf", use = ["edit", "view"] },

    # Python
    { name = "*.py", use = ["python", "edit", "view"] },

    # Shell scripts
    { name = "*.sh", use = ["shell", "edit", "view"] },
    { name = "*.bash", use = ["shell", "edit", "view"] },

    # Archives
    { name = "*.zip", use = "extract" },
    { name = "*.tar", use = "extract" },
    { name = "*.tar.gz", use = "extract" },
    { name = "*.tgz", use = "extract" },
    { name = "*.tar.bz2", use = "extract" },
    { name = "*.tar.xz", use = "extract" },
    { name = "*.7z", use = "extract" },
    { name = "*.rar", use = "extract" },

    # Fallback
    { name = "*", use = "open" },
]
'''

[[targets]]
target = "~/.config/yazi/theme.toml"
content = '''
# Yazi theme configuration using Kitchn colors

[manager]
cwd = { fg = "{{ colors.primary }}" }

# Hovered
hovered         = { fg = "{{ colors.bg }}", bg = "{{ colors.primary }}" }
preview_hovered = { underline = true }

# Find
find_keyword  = { fg = "{{ colors.orange }}", italic = true }
find_position = { fg = "{{ colors.magenta }}", bg = "reset", italic = true }

# Marker
marker_selected = { fg = "{{ colors.success }}", bg = "{{ colors.success }}" }
marker_copied   = { fg = "{{ colors.orange }}", bg = "{{ colors.orange }}" }
marker_cut      = { fg = "{{ colors.error }}", bg = "{{ colors.error }}" }

# Tab
tab_active   = { fg = "{{ colors.bg }}", bg = "{{ colors.primary }}" }
tab_inactive = { fg = "{{ colors.fg }}", bg = "{{ colors.selection_bg }}" }
tab_width    = 1

# Border
border_symbol = "│"
border_style  = { fg = "{{ colors.selection_bg }}" }

# Highlighting
syntect_theme = ""

[status]
separator_open  = ""
separator_close = ""
separator_style = { fg = "{{ colors.selection_bg }}", bg = "{{ colors.selection_bg }}" }

# Mode
mode_normal = { fg = "{{ colors.bg }}", bg = "{{ colors.primary }}", bold = true }
mode_select = { fg = "{{ colors.bg }}", bg = "{{ colors.orange }}", bold = true }
mode_unset  = { fg = "{{ colors.bg }}", bg = "{{ colors.error }}", bold = true }

# Progress
progress_label  = { fg = "{{ colors.fg }}", bold = true }
progress_normal = { fg = "{{ colors.primary }}", bg = "{{ colors.selection_bg }}" }
progress_error  = { fg = "{{ colors.error }}", bg = "{{ colors.selection_bg }}" }

# Permissions
permissions_t = { fg = "{{ colors.success }}" }
permissions_r = { fg = "{{ colors.orange }}" }
permissions_w = { fg = "{{ colors.error }}" }
permissions_x = { fg = "{{ colors.secondary }}" }
permissions_s = { fg = "{{ colors.selection_bg }}" }

[select]
border   = { fg = "{{ colors.primary }}" }
active   = { fg = "{{ colors.magenta }}" }
inactive = {}

[input]
border   = { fg = "{{ colors.primary }}" }
title    = {}
value    = {}
selected = { reversed = true }

[completion]
border   = { fg = "{{ colors.primary }}" }
active   = { bg = "{{ colors.selection_bg }}" }
inactive = {}

# Icons
icon_file = ""
icon_folder = ""
icon_command = ""

[tasks]
border  = { fg = "{{ colors.primary }}" }
title   = {}
hovered = { underline = true }

[which]
cols = 3
mask            = { bg = "{{ colors.bg }}" }
cand            = { fg = "{{ colors.secondary }}" }
rest            = { fg = "{{ colors.bright_black }}" }
desc            = { fg = "{{ colors.magenta }}" }
separator       = "  "
separator_style = { fg = "{{ colors.selection_bg }}" }

[help]
on      = { fg = "{{ colors.magenta }}" }
run     = { fg = "{{ colors.secondary }}" }
desc    = { fg = "{{ colors.fg }}" }
hovered = { bg = "{{ colors.selection_bg }}", bold = true }
footer  = { fg = "{{ colors.selection_bg }}", bg = "{{ colors.fg }}" }

[filetype]

rules = [
	# Images
	{ mime = "image/*", fg = "{{ colors.secondary }}" },

	# Videos
	{ mime = "video/*", fg = "{{ colors.orange }}" },
	{ mime = "audio/*", fg = "{{ colors.orange }}" },

	# Archives
	{ mime = "application/zip",             fg = "{{ colors.magenta }}" },
	{ mime = "application/gzip",            fg = "{{ colors.magenta }}" },
	{ mime = "application/x-tar",           fg = "{{ colors.magenta }}" },
	{ mime = "application/x-bzip",          fg = "{{ colors.magenta }}" },
	{ mime = "application/x-bzip2",         fg = "{{ colors.magenta }}" },
	{ mime = "application/x-7z-compressed", fg = "{{ colors.magenta }}" },
	{ mime = "application/x-rar",           fg = "{{ colors.magenta }}" },

	# Documents
	{ mime = "application/pdf", fg = "{{ colors.error }}" },

	# Fallback
	{ name = "*", fg = "{{ colors.fg }}" },
	{ name = "*/", fg = "{{ colors.primary }}" }
]
'''

[hooks]
