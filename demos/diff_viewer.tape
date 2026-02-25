# Diff Viewer Demo - unified and side-by-side diff display
Source demos/settings.tape
Output demos/gifs/diff_viewer.gif

Hide
Type "cargo run --example diff_viewer_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Scroll through diff
Down
Sleep 200ms
Down
Sleep 200ms
Down
Sleep 200ms
Down
Sleep 200ms

# Jump to next hunk
Type "]"
Sleep 600ms

# Toggle view mode (unified <-> side-by-side)
Type "v"
Sleep 1s

# Navigate changes
Type "n"
Sleep 500ms
Type "n"
Sleep 500ms

# Switch to another diff
Tab
Sleep 800ms

# Scroll new diff
Down
Sleep 200ms
Down
Sleep 200ms
Down
Sleep 200ms

# Jump to bottom
Type "G"
Sleep 800ms

Sleep 1s

Type "q"
