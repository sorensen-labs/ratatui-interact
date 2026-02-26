# Mouse Pointer Demo - pointer style indicator
# Note: VHS cannot script mouse movement; this demo shows keyboard controls
Source demos/settings.tape
Output demos/gifs/mouse_pointer.gif

Hide
Type "cargo run --example mouse_pointer_demo"
Enter
Sleep 5s
Show

Sleep 1s

# Cycle through pointer styles
Type "s"
Sleep 800ms
Type "s"
Sleep 800ms
Type "s"
Sleep 800ms
Type "s"
Sleep 800ms

# Toggle pointer off
Type "p"
Sleep 800ms

# Toggle back on
Type "p"
Sleep 800ms

# Cycle style once more
Type "s"
Sleep 1s

Sleep 1s

Type "q"
