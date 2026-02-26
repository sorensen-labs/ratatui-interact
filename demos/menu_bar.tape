# Menu Bar Demo - traditional dropdown menus with submenus
Source demos/settings.tape
Output demos/gifs/menu_bar.gif

Hide
Type "cargo run --example menu_bar_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Open File menu
Alt+f
Sleep 600ms

# Navigate items
Down
Sleep 300ms
Down
Sleep 300ms
Down
Sleep 300ms

# Open submenu
Right
Sleep 600ms

# Navigate submenu
Down
Sleep 300ms

# Close
Escape
Sleep 500ms

# Open Edit menu
Alt+e
Sleep 600ms
Down
Sleep 300ms
Down
Sleep 300ms
Escape
Sleep 400ms

# Cycle style
Type "t"
Sleep 600ms
Type "t"
Sleep 600ms

# Open menu again to show new style
Alt+f
Sleep 800ms
Escape
Sleep 500ms

Sleep 1s

Type "q"
