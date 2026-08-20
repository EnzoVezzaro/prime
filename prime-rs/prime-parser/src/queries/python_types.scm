; Python types query

; Type annotations
(type_annotation
  type: (_) @type)

; Return type
(function_definition
  return_type: (_) @type)

; Parameter types
(parameters
  (typed_parameter
    type: (_) @type))

; Variable annotations
(annotated_assignment
  annotation: (_) @type)

; Type aliases
(type_alias
  value: (_) @type)