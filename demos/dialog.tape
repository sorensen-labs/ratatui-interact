# Dialog Demo - popup dialog with input fields and checkboxes
Source demos/settings.tape
Output demos/gifs/dialog.gif

Hide
Type "cargo run --example dialog_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Open settings dialog
Type "s"
Sleep 800ms

# Tab to username field and type
Tab
Sleep 300ms
Type "CheeseLord"
Sleep 600ms

# Tab to email field
Tab
Sleep 300ms
Type "cheese@curds.io"
Sleep 600ms

# Tab to checkbox and toggle
Tab
Sleep 300ms
Space
Sleep 400ms

# Tab to next checkbox
Tab
Sleep 300ms
Space
Sleep 400ms

# Submit the dialog
Enter
Sleep 1s

Sleep 1s

Type "q"
