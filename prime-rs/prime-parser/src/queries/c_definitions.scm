; C definitions query

; Functions
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition

; Structs
(struct_specifier
  name: (type_identifier) @name) @definition

; Unions
(union_specifier
  name: (type_identifier) @name) @definition

; Enums
(enum_specifier
  name: (type_identifier) @name) @definition

; Typedefs
(type_definition
  declarator: (identifier) @name) @definition

; Enums constants
(enumerator
  name: (identifier) @name) @definition

; Global variables
(declaration
  declarator: (init_declarator
    declarator: (identifier) @name)) @definition