# TextArea Demo - multi-line text editor
Source demos/settings.tape
Output demos/gifs/textarea.gif

Hide
Type "cargo run --example textarea_demo"
Enter
Sleep 5s
Show

Sleep 800ms

# Type some text
Type "fn main() {"
Enter
Type "    println!("
Type "Hello"
Type ");"
Enter
Type "}"
Sleep 600ms

# Move cursor around
Up
Sleep 300ms
Up
Sleep 300ms
Left
Sleep 200ms
Left
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 400ms

# Toggle line numbers
Ctrl+L
Sleep 600ms
Ctrl+L
Sleep 600ms

# Add another line
Down
Sleep 200ms
Down
Sleep 200ms
Enter
Enter
Type "// Multi-line editing works great"
Sleep 600ms

# Delete a line
Ctrl+D
Sleep 600ms

Sleep 1s

Type "q"
