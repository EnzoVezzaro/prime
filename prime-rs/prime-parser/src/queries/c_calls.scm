; C calls query

(call_expression
  function: (identifier) @callee) @call

(call_expression
  function: (field_expression
    field: (field_identifier) @callee)) @call