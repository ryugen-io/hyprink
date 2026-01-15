[manifest]
id = "glow"
name = "glow"
version = "1.0.0"
description = "Beautiful terminal markdown viewer (replaces Frogmouth)"
authors = ["Ryugen"]

[[targets]]
target = "~/.config/fish/conf.d/kitchn_glow.fish"
content = """
# Kitchn Generated configuration for Glow
# Sets default paging and styling
alias md="glow -s dracula -p"
alias glow="glow -s dracula -p"
"""

[hooks]
info = "echo 'Switched to Glow! Run `md README.md` to see it.'"
