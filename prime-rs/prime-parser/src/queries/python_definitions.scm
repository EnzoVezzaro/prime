; Python definitions query

; Functions
(function_definition
  name: (identifier) @name) @definition

; Async functions
(async_function_definition
  name: (identifier) @name) @definition

; Classes
(class_definition
  name: (identifier) @name) @definition

; Lambda assigned to variable
(assignment
  left: (identifier) @name
  right: (lambda)) @definition

; Variables with type annotations
(annotated_assignment
  target: (identifier) @name) @definition

; Constants (ALL_CAPS convention)
(assignment
  left: (identifier) @name
  (#match? @name "^[A-Z_][A-Z0-9_]*$")) @definition