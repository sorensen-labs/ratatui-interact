# Copyable Pane Demo - scrollable content with view/copy mode
Source demos/settings.tape
Output demos/gifs/copyable_pane.gif

Hide
Type "cargo run --example copyable_pane_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Scroll down through content
Down
Sleep 200ms
Down
Sleep 200ms
Down
Sleep 200ms
Down
Sleep 200ms
Down
Sleep 200ms

# Page down
PageDown
Sleep 600ms
PageDown
Sleep 600ms

# Toggle line numbers
Type "n"
Sleep 800ms

# Toggle back
Type "n"
Sleep 800ms

# Scroll back up
Up
Sleep 200ms
Up
Sleep 200ms
Up
Sleep 200ms
Up
Sleep 200ms
Up
Sleep 200ms

Sleep 1s

Type "q"
