; Rust imports query
; Captures: @import (the import statement), @module (imported module), @item (imported item)

; use statements
(use_declaration
  argument: (use_tree) @import)

; extern crate
(extern_crate_declaration
  name: (identifier) @module)

; extern crate with rename
(extern_crate_declaration
  name: (identifier) @module
  rename: (identifier) @item)

; use with glob
(use_declaration
  argument: (use_tree
    (use_wildcard) @item))

; use with specific items
(use_declaration
  argument: (use_tree
    (identifier) @item))