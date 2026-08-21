; Go definitions query

; Functions
(function_declaration
  name: (identifier) @name
  (#set! @kind "function")) @definition

; Methods
(method_declaration
  name: (field_identifier) @name
  (#set! @kind "method")) @definition

; Types
(type_declaration
  (type_spec
    name: (type_identifier) @name
    (#set! @kind "type")) @definition

; Structs
(type_declaration
  (type_spec
    (struct_type) @name
    (#set! @kind "struct")) @definition)

; Interfaces
(type_declaration
  (type_spec
    (interface_type) @name
    (#set! @kind "interface")) @definition)

; Constants
(const_declaration
  (const_spec
    name: (identifier) @name
    (#set! @kind "constant")) @definition)

; Variables
(var_declaration
  (var_spec
    name: (identifier) @name
    (#set! @kind "variable")) @definition)

; Functions with receivers (methods on types)
(method_declaration
  receiver: (parameter_list)
  name: (field_identifier) @name
  (#set! @kind "method")) @definition