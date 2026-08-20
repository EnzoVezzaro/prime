; Java definitions query

; Classes
(class_declaration
  name: (identifier) @name) @definition

; Interfaces
(interface_declaration
  name: (identifier) @name) @definition

; Enums
(enum_declaration
  name: (identifier) @name) @definition

; Methods
(method_declaration
  name: (identifier) @name) @definition

; Constructors
(constructor_declaration
  name: (identifier) @name) @definition

; Fields
(field_declaration
  (variable_declarator
    name: (identifier) @name)) @definition

; Enums constants
(enum_constant
  name: (identifier) @name) @definition

; Annotations
(annotation_type_declaration
  name: (identifier) @name) @definition

; Records
(record_declaration
  name: (identifier) @name) @definition