# Mylodon notes

Notes on Mylodon interfaces, default keybindings

## Nav mode

Main nav keys (j/k/d/f/h/l/w/W/e/E/{/}) can be combined with a number (e.g. 5 {) to move 5 {'s into the file.
Other keys could be set to find custom strings.

"contents" below refers to characters inside "", '', ``, (), {}, <>, or custom start/end

| Key                           | Raw                           | Mod/shift                 | Ctrl/alt              | Double-tap            |
| ---                           | ---                           | ---                       | ---                   | ---                   |
| j/k, left/right               | up/down                       | reverse                   | x5                    | start/end doc         |
| h/l, up/down                  | left/right                    | reverse                   | x5                    | start/end line        |
| w                             | next word                     | prev word                 | x5                    | last word in line     |
| e                             | next end word                 | prev end word             | x5                    | last end word in line |
| {sym}                         | next symbol                   | prev symbol               | x5                    | last symbol in line   |
| SPACE / leader                | open leader menu              | Quick 1: Save             | Quick 2: Save all     | Quick 3: Open         |
| g                             | GET MODE (selected or type)   | GET MODE (replace)        |                       |                       |
| Enter                         | EDIT MODE (insert)            | EDIT MODE (append)        | EDIT MODE (prepend)   | EDIT MODE (next line) |
|                               |                               | double-tap - append eol   | double - prepend line |                       |
| s                             | SELECT (highlight) MODE       | SELECT MODE (to eol)      | SELECT MODE (contents)| Select line           |
| c, p, x, r, d - selectd or mod| copy, paste, cut, replace, del| * (to end of line)        | * (contents)          | * (line)              |
| Ctrl-c, -v, -a, -x, del, bsp  | Common use                    |                           |                       |                       |
| ESC                           | Clear sel, search mode NAV    |                           |                       |                       |

## GET MODE

If highlighting, Get Mode is initialized to find highlighted word. Can also initialize into find/replace mode.

h/j/k/l/w/e or arrow keys now cycle through prev/next match (left/right are prev, next match; up/down are prev/next on next or prev line)

g cancels Get Mode and returns to previous position. Double-tap g cancels Get Mode and moves cursor to selection (word is selected). Shift+G toggles replace mode. Enter selects current and moves position, starts Edit Mode (mods valid).

In standard mode: r replaces word and enters edit mode. In replace mode: r replaces the word with the replace word and moves to the next match.

Space enters Get Mode commands: replace all, multi-file, next file, match case, regex, etc.

## SELECT MODE

Movement keys and Get Mode work as they have (with limited functionality scoped to just selecting text). s exits Select Mode.

## Editing Mode

Can jump in at cursor pos, after word (append), before word (prepend), next line, end of line (append), beginning of line (prepend)

SHIFT + ENTER returns to NAV mode
