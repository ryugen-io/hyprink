[meta]
id = "waybar"

[[templates]]
target = "~/.config/waybar/style.css"
content = """
* {
    font-family: "{{ fonts.ui }}";
    font-size: {{ fonts.size_ui }}px;
}
window#waybar {
    background-color: {{ colors.bg }};
    border-bottom: 2px solid {{ colors.primary }};
}
"""

[hooks]
reload = "pkill -SIGUSR2 waybar"
