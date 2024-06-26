# Mylodon

I love the idea of Vim, but I don't like Vim. It feels less like intuitive keyboard-based navigation and more like arcane magic, with various spells meant to be committed to memory, dark catacombs stuffed with books filled with keyboard shortcuts that are nothing like the keyboard shortcuts you've ever used before.

That's fine -- I can get used to a little arcane magic here and there -- but what really ruined it for me was that the promise of rapid, hands-on-keyboard navigation was significantly hampered by the use of symbol keys tucked way off in the periphery of your typical QWERTY keyboard -- ^ for start of line, $ for end of line, % for search, esc to get back to navigation... It works for a lot of folks, but it didn't work for me, and not because I didn't give it time. It just felt like the folks that loved Vim spent most of their time optimizing key remappings or digging deep into the Vim apocryphal man pages, or that they had some other trick to make Vim feel more like home: DVORAK keyboards or some other alternative layout, or ergonomic keyboards that put every key mere millimeters from a finger.

The goal of this project is to take all the great things about Vim and strip out all the unnecessary complexity. Keep things on the home row as much as possible, shoot for maximum ergonomics, consistency in navigation modifiers, no goofy finger gymnastics, and navigation keys that stick close to well-known shortcuts or that are easily mnemonicized. Where those goals can't be reached for a particular individual, I aim to provide maximum options for customization and extensibility so you can take this simple base and make it your own.

Look, if you use and love Vim, great. It's a wonderful tool, and a radically different way to approach text editing that the rest of the world should never have abandoned. The truth is, the best tool for you is the tool you're most productive in. If that's Vim, this isn't for you. It's for me... and maybe it's for someone else out there, too. Give it a shot.

## Mode switches

* Enter - enter insertion mode
* Shift + Enter - exit to nav mode

## Navigation mode

In general, navigation keys move you forward through the text; keys modified with Shift move you backward. Mod/ctrl allows you to specify a number of times to repeat the command.

* j - move down     (mod/ctrl - jump to end)
* k - move up       (mod/ctrl - jump to beginning)
* f - move right    (mod/ctrl - jump to end of line)
* d - move left     (mod/ctrl - jump to beginning of line)
* w - move to start of next word
* W - move to start of last word
* e - move to end of next word
* E - move to end of last word

### Editing

* Enter or i - enter insert mode at current position
* a - append
* p - prepend
* r - enter replace submode
* d - enter delete submode
* c - enter copy submode
* x - enter cut submode
* v - enter paste submode
* g - enter search-in-line mode
* G - enter search-in-document mode

## Submodes