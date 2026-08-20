; Rust definitions query
; Captures: @definition (the node), @name (identifier)

; Functions
(function_item
  name: (identifier) @name) @definition

; Async functions
(async_item
  (function_item
    name: (identifier) @name)) @definition

; Methods in impl blocks
(impl_item
  (function_item
    name: (identifier) @name)) @definition

; Structs
(struct_item
  name: (type_identifier) @name) @definition

; Enums
(enum_item
  name: (type_identifier) @name) @definition

; Traits
(trait_item
  name: (type_identifier) @name) @definition

; Impl blocks (for inherent impls)
(impl_item
  type: (type_identifier) @name) @definition

; Constants
(const_item
  name: (identifier) @name) @definition

; Static variables
(static_item
  name: (identifier) @name) @definition

; Type aliases
(type_item
  name: (type_identifier) @name) @definition

; Modules
(mod_item
  name: (identifier) @name) @definition

; Macros
(macro_definition
  name: (identifier) @name) @definition

; Macros (declarative)
(macro_rules
  name: (identifier) @name) @definition