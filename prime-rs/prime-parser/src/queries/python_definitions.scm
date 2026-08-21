; Python definitions query

; Functions
(function_definition
  name: (identifier) @name
  (#set! @kind "function")) @definition

; Async functions
(async_function_definition
  name: (identifier) @name
  (#set! @kind "function")) @definition

; Classes
(class_definition
  name: (identifier) @name
  (#set! @kind "class")) @definition

; Lambda assigned to variable
(assignment
  left: (identifier) @name
  right: (lambda)
  (#set! @kind "function")) @definition

; Variables with type annotations
(annotated_assignment
  target: (identifier) @name
  (#set! @kind "variable")) @definition

; Constants (ALL_CAPS convention)
(assignment
  left: (identifier) @name
  (#match? @name "^[A-Z_][A-Z0-9_]*$")
  (#set! @kind "constant")) @definition

; Decorated functions
(decorated_definition
  (function_definition
    name: (identifier) @name
    (#set! @kind "function")) @definition)