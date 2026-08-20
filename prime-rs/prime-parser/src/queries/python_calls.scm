; Python calls query

; Function calls
(call
  function: (identifier) @callee) @call

; Method calls
(call
  function: (attribute
    attribute: (identifier) @callee)) @call

; Constructor calls
(call
  function: (identifier) @callee) @call