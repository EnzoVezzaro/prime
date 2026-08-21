; TypeScript definitions query

; Functions
(function_declaration
  name: (identifier) @name
  (#set! @kind "function")) @definition

; Arrow functions assigned to variables
(variable_declaration
  (variable_declarator
    name: (identifier) @name
    value: (arrow_function) @value
    (#set! @kind "function")) @definition

; Function expressions
(variable_declaration
  (variable_declarator
    name: (identifier) @name
    value: (function_expression) @value
    (#set! @kind "function")) @definition

; Methods in classes
(method_definition
  name: (property_identifier) @name
  (#set! @kind "method")) @definition

; Classes
(class_declaration
  name: (type_identifier) @name
  (#set! @kind "class")) @definition

; Interfaces
(interface_declaration
  name: (type_identifier) @name
  (#set! @kind "interface")) @definition

; Type aliases
(type_alias_declaration
  name: (type_identifier) @name
  (#set! @kind "type_alias")) @definition

; Enums
(enum_declaration
  name: (identifier) @name
  (#set! @kind "enum")) @definition

; Variables (const/let)
(variable_declaration
  (variable_declarator
    name: (identifier) @name
    (#set! @kind "variable")) @definition