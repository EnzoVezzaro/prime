; Rust calls query
; Captures: @call (call expression), @callee (called function), @caller (containing function)

; Function calls
(call_expression
  function: [
    (identifier) @callee
    (scoped_identifier) @callee
    (field_expression) @callee
  ] @call)

; Method calls
(call_expression
  function: (field_expression
    receiver: (_) @receiver
    field: (field_identifier) @callee)) @call

; Macro calls
(macro_invocation
  macro: (identifier) @callee) @call

; Method calls with turbofish
(call_expression
  function: (generic_args
    (scoped_identifier) @callee)) @call