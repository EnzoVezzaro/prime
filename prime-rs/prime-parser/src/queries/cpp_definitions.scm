; C++ definitions query (extends C)

; Classes
(class_specifier
  name: (type_identifier) @name) @definition

; Structs (also in C)
(struct_specifier
  name: (type_identifier) @name) @definition

; Namespaces
(namespace_definition
  name: (identifier) @name) @definition

; Templates
(template_declaration
  (class_specifier
    name: (type_identifier) @name)) @definition

(template_declaration
  (function_definition
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition

; Using declarations
(using_declaration
  name: (identifier) @name) @definition

; Type aliases
(type_definition
  declarator: (type_identifier) @name) @definition

; Concepts
(concept_definition
  name: (identifier) @name) @definition