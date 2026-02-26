# Tab View Demo - tabbed interface with multiple positions
Source demos/settings.tape
Output demos/gifs/tab_view.gif

Hide
Type "cargo run --example tab_view_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Navigate tabs
Right
Sleep 500ms
Right
Sleep 500ms
Right
Sleep 500ms

# Jump to tab by number
Type "1"
Sleep 600ms
Type "3"
Sleep 600ms
Type "5"
Sleep 600ms

# Cycle tab position (Top -> Right)
Type "p"
Sleep 800ms

# Navigate in new position
Down
Sleep 400ms
Down
Sleep 400ms

# Cycle to Bottom
Type "p"
Sleep 800ms

# Cycle to Left
Type "p"
Sleep 800ms

# Back to Top
Type "p"
Sleep 800ms

Sleep 1s

Type "q"
