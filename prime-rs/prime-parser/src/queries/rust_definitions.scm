; Rust definitions query

; Functions
(function_item
  name: (identifier) @name
  body: (block) @definition)

; Async functions
(function_item
  name: (identifier) @name
  body: (block) @definition)

; Structs
(struct_item
  name: (type_identifier) @name
  body: (field_declaration_list) @definition)

; Enums
(enum_item
  name: (type_identifier) @name
  body: (enum_variant_list) @definition)

; Traits
(trait_item
  name: (type_identifier) @name
  body: (trait_block) @definition)

; Impl blocks
(impl_item
  type: (type_identifier) @name
  body: (declaration_list) @definition)

; Constants
(const_item
  name: (identifier) @name
  type: (_) @definition)

; Static variables
(static_item
  name: (identifier) @name
  type: (_) @definition)

; Type aliases
(type_item
  name: (type_identifier) @name
  type: (_) @definition)

; Modules
(mod_item
  name: (identifier) @name) @definition

; Macros
(macro_definition
  name: (identifier) @name) @definition

; Macros (declarative)
(macro_rules
  name: (identifier) @name) @definition
