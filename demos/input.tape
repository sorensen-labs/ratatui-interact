# Input Demo - single-line text input fields
Source demos/settings.tape
Output demos/gifs/input.gif

Hide
Type "cargo run --example input_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Type in name field
Type "Jane Doe"
Sleep 600ms

# Tab to email
Tab
Sleep 400ms
Type "jane@example.com"
Sleep 600ms

# Tab to message
Tab
Sleep 400ms
Type "Hello from ratatui-interact!"
Sleep 600ms

# Go back and edit name
Shift+Tab
Sleep 300ms
Shift+Tab
Sleep 300ms

# Show cursor movement
Left
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms

# Delete characters
Backspace
Sleep 200ms
Backspace
Sleep 200ms
Backspace
Sleep 200ms

# Re-type
Type "ith"
Sleep 600ms

Sleep 1s

Type "q"
