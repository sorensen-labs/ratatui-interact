# Explorer & Log Demo - file explorer with log viewer and toasts
Source demos/settings.tape
Output demos/gifs/explorer_log.gif

Hide
Type "cargo run --example explorer_log_demo --features filesystem"
Enter
Sleep 5s
Show

Sleep 800ms

# Navigate file explorer
Down
Sleep 300ms
Down
Sleep 300ms
Down
Sleep 300ms

# Open a directory
Enter
Sleep 600ms

# Navigate inside
Down
Sleep 300ms
Down
Sleep 300ms

# Go back up
Enter
Sleep 600ms

# Select files
Space
Sleep 400ms
Down
Sleep 300ms
Space
Sleep 400ms

# Switch to log panel
Tab
Sleep 600ms

# Scroll log
Up
Sleep 300ms
Up
Sleep 300ms

Sleep 1s

Type "q"
