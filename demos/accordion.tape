# Accordion Demo - collapsible sections (single & multi expand)
Source demos/settings.tape
Output demos/gifs/accordion.gif

Hide
Type "cargo run --example accordion_demo"
Enter
Sleep 5s
Show

# Start in FAQ panel (single-expand mode)
Sleep 1s

# Navigate and expand sections
Down
Sleep 400ms
Enter
Sleep 800ms
Down
Sleep 400ms
Enter
Sleep 800ms

# Switch to Settings panel (multi-expand mode)
Tab
Sleep 600ms

# Expand multiple sections
Enter
Sleep 400ms
Down
Sleep 300ms
Enter
Sleep 400ms
Down
Sleep 300ms
Enter
Sleep 800ms

# Show all expanded
Sleep 1s

Type "q"
