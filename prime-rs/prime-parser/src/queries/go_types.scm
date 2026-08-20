; Go types query

; Type specifications
(type_spec
  name: (type_identifier) @name
  type: (_) @type)

; Function signatures
(function_declaration
  result: (parameter_list) @type)

; Method signatures
(method_declaration
  result: (parameter_list) @type)

; Variable declarations with types
(var_declaration
  (var_spec
    type: (_) @type))

; Parameter types
(parameter_list
  (parameter_declaration
    type: (_) @type))