[manifest]
name = "wired"
version = "0.1.0"
authors = ["Kitchn"]
description = "Wired notification daemon theme and layout"
license = "MIT"

[[targets]]
target = "~/.config/wired/wired.ron"
content = """
(
    max_notifications: 0,
    timeout: 5000,
    poll_interval: 16,
    
    // Layout blocks
    layout_blocks: [
        // Root Notification Block
        (
            name: "root",
            parent: "",
            hook: Hook(parent_anchor: BL, self_anchor: BL),
            offset: Vec2(x: 0.0, y: -15.0), // User requested 15
            params: NotificationBlock((
                monitor: 0,
                border_width: 2.0,
                border_rounding: 6.0,
                gap: Vec2(x: 0.0, y: 12.0), 
                notification_hook: Hook(parent_anchor: TL, self_anchor: BL), // Stack upwards
                background_color: Color(hex: "{{ colors.bg }}"),
                border_color: Color(hex: "{{ colors.primary }}"),
                border_color_low: Color(hex: "{{ colors.black }}"),
                border_color_critical: Color(hex: "{{ colors.error }}"),
                border_color_paused: Color(hex: "{{ colors.warn }}"),
            )),
        ),

        // Image Block (Icon)
        (
            name: "image",
            parent: "root",
            hook: Hook(parent_anchor: TL, self_anchor: TL),
            offset: Vec2(x: 12.0, y: 12.0),
            params: ImageBlock((
                image_type: Hint,
                padding: Padding(left: 0.0, right: 0.0, top: 0.0, bottom: 0.0),
                rounding: 4.0,
                scale_width: 56,
                scale_height: 56,
                filter_mode: Lanczos3,
            )),
        ),

        // Summary Text (Title)
        (
            name: "summary",
            parent: "image",
            hook: Hook(parent_anchor: TR, self_anchor: TL),
            offset: Vec2(x: 12.0, y: 0.0),
            params: TextBlock((
                text: "%s",
                padding: Padding(left: 0.0, right: 10.0, top: 0.0, bottom: 0.0),
                font: "{{ fonts.ui }} Bold 13",
                color: Color(hex: "{{ colors.primary }}"),
                dimensions: (width: (min: 50, max: 350), height: (min: 0, max: 0)),
                ellipsize: Middle,
            )),
        ),

        // Body Text
        (
            name: "body",
            parent: "summary",
            hook: Hook(parent_anchor: BL, self_anchor: TL),
            offset: Vec2(x: 0.0, y: 6.0),
            params: TextBlock((
                text: "%b",
                padding: Padding(left: 0.0, right: 10.0, top: 0.0, bottom: 4.0),
                font: "{{ fonts.ui }} 12",
                color: Color(hex: "{{ colors.fg }}"),
                dimensions: (width: (min: 50, max: 350), height: (min: 0, max: 0)),
                ellipsize: Middle,
            )),
        ),
    ],
)
"""
