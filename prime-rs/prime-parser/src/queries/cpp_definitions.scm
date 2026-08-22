; C++ definitions query - Simplified for compatibility

; Class definitions
(class_specifier name: (type_identifier) @name) @definition

; Struct definitions
(struct_specifier name: (type_identifier) @name) @definition

; Enum definitions
(enum_specifier name: (type_identifier) @name) @definition

; Function definitions
(function_declarator declarator: (identifier) @name) @definition

; Method definitions (member functions)
(function_declarator 
  declarator: (field_identifier) @name) @definition

; Namespace definitions
(namespace_definition name: (namespace_identifier) @name) @definition

; Type alias definitions
(alias_declaration name: (type_identifier) @name) @definition

; Top-level variable definitions (in translation_unit scope)
(translation_unit
  (declaration 
    declarator: (identifier) @name)) @definition

; Top-level field definitions (in class/struct bodies)
(field_declaration 
  declarator: (field_identifier) @name) @definition
