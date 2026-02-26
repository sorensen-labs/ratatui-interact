# Hotkey Dialog Demo - searchable keyboard shortcut reference
Source demos/settings.tape
Output demos/gifs/hotkey_dialog.gif

Hide
Type "cargo run --example hotkey_dialog_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Open hotkey dialog
Type "h"
Sleep 800ms

# Navigate categories
Down
Sleep 400ms
Down
Sleep 400ms
Down
Sleep 400ms

# Tab to search and filter
Tab
Sleep 400ms
Type "save"
Sleep 800ms

# Clear search and try another
Ctrl+U
Sleep 300ms
Type "copy"
Sleep 800ms

# Close dialog
Escape
Sleep 600ms

Sleep 1s

Type "q"
