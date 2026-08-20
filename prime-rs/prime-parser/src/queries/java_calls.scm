; Java calls query

; Method calls
(method_invocation
  name: (identifier) @callee) @call

; Constructor calls
(object_creation_expression
  type: (type_identifier) @callee) @call

; Super calls
(super_constructor_invocation) @call