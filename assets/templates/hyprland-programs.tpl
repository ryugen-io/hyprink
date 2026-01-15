[manifest]
name = "hyprland-programs"
version = "0.1.0"
authors = ["Kitchn"]
description = "Hyprland program definitions (terminal, menu, file manager)"
license = "MIT"

[[targets]]
target = "~/.config/hypr/conf.d/programs.conf"
content = '''
###################
### MY PROGRAMS ###
###################

# See https://wiki.hypr.land/Configuring/Keywords/

# Set programs that you use
$terminal = rio -e zellij
$fileManager = yazi
$menu = fuzzel
'''

[hooks]
