; Go definitions query

; Functions
(function_declaration
  name: (identifier) @name) @definition

; Methods
(method_declaration
  name: (field_identifier) @name) @definition

; Types
(type_declaration
  (type_spec
    name: (type_identifier) @name)) @definition

; Constants
(const_declaration
  (const_spec
    name: (identifier) @name)) @definition

; Variables
(var_declaration
  (var_spec
    name: (identifier) @name)) @definition