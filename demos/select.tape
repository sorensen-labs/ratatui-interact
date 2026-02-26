# Select Demo - dropdown selection component
Source demos/settings.tape
Output demos/gifs/select.gif

Hide
Type "cargo run --example select_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Open first dropdown (Color)
Enter
Sleep 500ms
Down
Sleep 300ms
Down
Sleep 300ms
Enter
Sleep 600ms

# Tab to next select (Size)
Tab
Sleep 400ms
Enter
Sleep 500ms
Down
Sleep 300ms
Enter
Sleep 600ms

# Tab to next select (Priority)
Tab
Sleep 400ms
Space
Sleep 500ms
Down
Sleep 300ms
Down
Sleep 300ms
Enter
Sleep 600ms

# Go back to first and change selection
Shift+Tab
Sleep 300ms
Shift+Tab
Sleep 300ms
Enter
Sleep 500ms
Down
Sleep 200ms
Down
Sleep 200ms
Down
Sleep 200ms
Enter
Sleep 600ms

Sleep 1s

Type "q"
