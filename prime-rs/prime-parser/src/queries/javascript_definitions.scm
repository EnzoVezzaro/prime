; JavaScript definitions query

; Functions
(function_declaration
  name: (identifier) @name) @definition

; Arrow functions assigned to variables
(variable_declaration
  (variable_declarator
    name: (identifier) @name
    value: (arrow_function) @value)) @definition

; Function expressions
(variable_declaration
  (variable_declarator
    name: (identifier) @name
    value: (function_expression) @value)) @definition

; Methods in classes
(method_definition
  name: (property_identifier) @name) @definition

; Classes
(class_declaration
  name: (identifier) @name) @definition

; Variables (const/let/var)
(variable_declaration
  (variable_declarator
    name: (identifier) @name)) @definition

; Object methods
(pair
  key: (property_identifier) @name
  value: (function_expression)) @definition