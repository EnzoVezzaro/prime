; C types query

; Type specifiers
(type_identifier) @type

; Primitive types
(primitive_type) @type

; Pointer types
(pointer_type) @type

; Array types
(array_declarator) @type

; Function pointer types
(pointer_declarator
  declarator: (function_declarator)) @type

; Struct/union/enum types
(struct_specifier) @type
(union_specifier) @type
(enum_specifier) @type

; Typedef names
(type_identifier) @type

; Parameter types
(parameter_declaration
  type: (_) @type)

; Return types
(function_declarator
  type: (_) @type)