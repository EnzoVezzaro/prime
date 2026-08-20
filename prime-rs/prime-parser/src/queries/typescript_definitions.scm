; TypeScript definitions query

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
  name: (type_identifier) @name) @definition

; Interfaces
(interface_declaration
  name: (type_identifier) @name) @definition

; Type aliases
(type_alias_declaration
  name: (type_identifier) @name) @definition

; Enums
(enum_declaration
  name: (identifier) @name) @definition

; Variables (const/let)
(variable_declaration
  (variable_declarator
    name: (identifier) @name)) @definition