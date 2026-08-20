; TypeScript calls query

; Function calls
(call_expression
  function: (identifier) @callee) @call

; Method calls
(call_expression
  function: (member_expression
    property: (property_identifier) @callee)) @call

; New expressions
(new_expression
  constructor: (identifier) @callee) @call

; Tagged template literals
(tagged_template_expression
  tag: (identifier) @callee) @call