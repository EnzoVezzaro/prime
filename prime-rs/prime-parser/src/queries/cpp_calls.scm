; C++ calls query (extends C)

; Method calls
(call_expression
  function: (field_expression
    field: (field_identifier) @callee)) @call

; Template calls
(call_expression
  function: (template_function
    name: (identifier) @callee)) @call

; Operator calls
(call_expression
  function: (operator_name) @callee) @call

; New/delete
(new_expression
  type: (_) @callee) @call

(delete_expression
  argument: (_) @callee) @call