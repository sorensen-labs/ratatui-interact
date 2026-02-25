# Split Pane Demo - resizable panels with drag support
Source demos/settings.tape
Output demos/gifs/split_pane.gif

Hide
Type "cargo run --example split_pane_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Resize pane - move divider right
Right
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms
Sleep 400ms

# Move divider left
Left
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms
Sleep 400ms

# Toggle orientation
Type "o"
Sleep 800ms

# Resize vertically
Down
Sleep 200ms
Down
Sleep 200ms
Down
Sleep 200ms
Sleep 400ms

# Reset to 50%
Type "r"
Sleep 600ms

# Toggle back
Type "o"
Sleep 600ms

# Switch focus
Tab
Sleep 500ms
Tab
Sleep 500ms

Sleep 1s

Type "q"
