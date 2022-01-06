# .preonapp File Format

A text-based format parsed at compile-time.

```rs
vbox(expand: "both") {
    panel(color: "#da0037", expand: "horizontal", min_size: (0, 100));
    hbox(expand: "both") {
        panel(expand: "vertical", min_size: (300, 0), padding: (16)) {
            vbox(fit: "vertical", expand: "horizontal") {
                panel(color: "#c4c4c4", expand: "horizontal", min_size: (0, 48));
                panel(color: "#c4c4c4", expand: "horizontal", min_size: (0, 48));
                panel(color: "#c4c4c4", expand: "horizontal", min_size: (0, 48));
                label(text: "Hello, World", expand: "horizontal", min_size: (0, 48))
            }
        };
        panel(expand: "both", color: "#d3d3d3")
    }
}
```