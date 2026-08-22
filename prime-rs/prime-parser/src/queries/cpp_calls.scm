; C++ calls query

; Function calls via identifier
(call_expression
  function: (identifier) @callee) @call

; Method calls via field access (obj.method())
(call_expression
  function: (field_expression
    field: (field_identifier) @callee)) @call

; Template function calls
(call_expression
  function: (template_function
    name: (identifier) @callee)) @call

; Constructor calls (ClassName())
(call_expression
  function: (type_identifier) @callee) @call
