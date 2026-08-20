; Java types query

; Type declarations
(type_identifier) @type

; Class types
(class_type
  name: (type_identifier) @type)

; Interface types
(interface_type
  name: (type_identifier) @type)

; Array types
(array_type
  type: (_) @type)

; Generic types
(generic_type
  type: (type_identifier) @type
  type_arguments: (type_arguments) @args)

; Parameter types
(formal_parameter
  type: (_) @type)

; Return types
(method_declaration
  type: (_) @type)

; Field types
(field_declaration
  type: (_) @type)

; Local variable types
(local_variable_declaration
  type: (_) @type)

; Cast expressions
(cast_expression
  type: (_) @type)