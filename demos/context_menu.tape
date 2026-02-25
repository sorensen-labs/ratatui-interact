# Context Menu Demo - right-click popup menus with submenus
Source demos/settings.tape
Output demos/gifs/context_menu.gif

Hide
Type "cargo run --example context_menu_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Navigate file list
Down
Sleep 400ms
Down
Sleep 400ms
Down
Sleep 400ms

# Open context menu (Enter or application key)
Enter
Sleep 600ms

# Navigate menu items
Down
Sleep 400ms
Down
Sleep 400ms

# Open submenu
Right
Sleep 600ms

# Navigate submenu
Down
Sleep 400ms

# Select
Enter
Sleep 800ms

# Close/escape
Escape
Sleep 500ms

Sleep 1s

Type "q"
