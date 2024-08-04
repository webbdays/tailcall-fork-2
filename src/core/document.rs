use async_graphql::parser::types::*;
use async_graphql::{Pos, Positioned};
use async_graphql_value::{ConstValue, Name};

fn pos<A>(a: A) -> Positioned<A> {
    Positioned::new(a, Pos::default())
}

struct LineBreaker<'a> {
    string: &'a str,
    break_at: usize,
    index: usize,
}

impl<'a> LineBreaker<'a> {
    fn new(string: &'a str, break_at: usize) -> Self {
        LineBreaker { string, break_at, index: 0 }
    }
}

impl<'a> Iterator for LineBreaker<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.string.len() {
            return None;
        }

        let end_index = self
            .string
            .chars()
            .skip(self.index + self.break_at)
            .enumerate()
            .find(|(_, ch)| ch.is_whitespace())
            .map(|(index, _)| self.index + self.break_at + index + 1)
            .unwrap_or(self.string.len());

        let start_index = self.index;
        self.index = end_index;

        Some(&self.string[start_index..end_index])
    }
}

fn get_formatted_docs(docs: Option<String>, indent: usize) -> String {
    let mut indent_str = String::new();
    if indent > 0 {
        indent_str = " ".repeat(indent);
    }
    let mut formatted_docs = String::new();
    if let Some(docs) = docs {
        let docs: String = docs.chars().filter(|ch| ch != &'\n').collect();
        let line_breaker = LineBreaker::new(&docs, 80);
        formatted_docs.push_str(format!("{}\"\"\"", indent_str).as_str());
        for line in line_breaker {
            formatted_docs.push_str(format!("\n{}{}", indent_str, line).as_str());
        }
        formatted_docs.push_str(format!("\n{}\"\"\"\n", indent_str).as_str());
    }

    formatted_docs
}

pub fn print_directives<'a>(directives: impl Iterator<Item = &'a ConstDirective>) -> String {
    directives
        .map(|d| print_directive(&const_directive_to_sdl(d)))
        .collect::<Vec<String>>()
        .join(" ")
}

fn print_pos_directives(directives: &[Positioned<ConstDirective>]) -> String {
    let mut output = print_directives(directives.iter().map(|directive| &directive.node));

    if !output.is_empty() {
        output.push(' ');
    }

    output
}

fn print_schema(schema: &SchemaDefinition) -> String {
    let directives = print_pos_directives(&schema.directives);

    let query = schema
        .query
        .as_ref()
        .map_or(String::new(), |q| format!("  query: {}\n", q.node));
    let mutation = schema
        .mutation
        .as_ref()
        .map_or(String::new(), |m| format!("  mutation: {}\n", m.node));
    let subscription = schema
        .subscription
        .as_ref()
        .map_or(String::new(), |s| format!("  subscription: {}\n", s.node));
    if mutation.is_empty() && query.is_empty() {
        return String::new();
    }
    format!(
        "schema {}{{\n{}{}{}}}\n",
        directives, query, mutation, subscription
    )
}

fn const_directive_to_sdl(directive: &ConstDirective) -> DirectiveDefinition {
    DirectiveDefinition {
        description: None,
        name: pos(Name::new(directive.name.node.as_str())),
        arguments: directive
            .arguments
            .iter()
            .filter_map(|(k, v)| {
                if v.node != ConstValue::Null {
                    Some(pos(InputValueDefinition {
                        description: None,
                        name: pos(Name::new(k.node.clone())),
                        ty: pos(Type {
                            nullable: true,
                            base: async_graphql::parser::types::BaseType::Named(Name::new(
                                v.to_string(),
                            )),
                        }),
                        default_value: Some(pos(ConstValue::String(v.to_string()))),
                        directives: Vec::new(),
                    }))
                } else {
                    None
                }
            })
            .collect(),
        is_repeatable: true,
        locations: vec![],
    }
}

fn print_type_def(type_def: &TypeDefinition) -> String {
    match &type_def.kind {
        TypeKind::Scalar => {
            let doc = get_formatted_docs(type_def.description.as_ref().map(|d| d.node.clone()), 0);
            format!("{}scalar {}\n", doc, type_def.name.node)
        }
        TypeKind::Union(union) => {
            format!(
                "union {} = {}\n",
                type_def.name.node,
                union
                    .members
                    .iter()
                    .map(|name| name.node.clone())
                    .collect::<Vec<_>>()
                    .join(" | ")
            )
        }
        TypeKind::InputObject(input) => {
            let directives = print_pos_directives(&type_def.directives);
            let doc = get_formatted_docs(type_def.description.as_ref().map(|d| d.node.clone()), 0);
            format!(
                "{}input {} {}{{\n{}\n}}\n",
                doc,
                type_def.name.node,
                directives,
                input
                    .fields
                    .iter()
                    .map(|f| print_input_value(&f.node))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
        TypeKind::Interface(interface) => {
            let implements = if !interface.implements.is_empty() {
                format!(
                    "implements {} ",
                    interface
                        .implements
                        .iter()
                        .map(|name| name.node.clone())
                        .collect::<Vec<_>>()
                        .join(" & ")
                )
            } else {
                String::new()
            };
            format!(
                "interface {} {}{{\n{}\n}}\n",
                type_def.name.node,
                implements,
                interface
                    .fields
                    .iter()
                    .map(|f| print_field(&f.node))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
        TypeKind::Object(object) => {
            let implements = if !object.implements.is_empty() {
                format!(
                    "implements {} ",
                    object
                        .implements
                        .iter()
                        .map(|name| name.node.clone())
                        .collect::<Vec<_>>()
                        .join(" & ")
                )
            } else {
                String::new()
            };
            let directives = print_pos_directives(&type_def.directives);
            let doc = type_def.description.as_ref().map_or(String::new(), |d| {
                format!(r#"  """{}  {}{}  """{}"#, "\n", d.node, "\n", "\n")
            });
            format!(
                "{}type {} {}{}{{\n{}\n}}\n",
                doc,
                type_def.name.node,
                implements,
                directives,
                object
                    .fields
                    .iter()
                    .map(|f| print_field(&f.node))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
        TypeKind::Enum(en) => {
            let directives = print_pos_directives(&type_def.directives);
            let enum_def = format!(
                "enum {} {}{{\n{}\n}}\n",
                type_def.name.node,
                directives,
                en.values
                    .iter()
                    .map(|v| print_enum_value(&v.node))
                    .collect::<Vec<String>>()
                    .join("\n")
            );

            if let Some(desc) = &type_def.description {
                let ds = format!("\"\"\"\n{}\n\"\"\"\n", desc.node.as_str());
                ds + &enum_def
            } else {
                enum_def
            }
        } // Handle other type kinds...
    }
}

fn print_enum_value(value: &async_graphql::parser::types::EnumValueDefinition) -> String {
    let directives_str = print_pos_directives(&value.directives);
    if directives_str.is_empty() {
        format!("  {}", value.value)
    } else {
        format!("  {} {}", value.value, directives_str)
    }
}

fn print_field(field: &async_graphql::parser::types::FieldDefinition) -> String {
    let directives = print_pos_directives(&field.directives);
    let args_str = if !field.arguments.is_empty() {
        let args = field
            .arguments
            .iter()
            .map(|arg| {
                let nullable = if arg.node.ty.node.nullable { "" } else { "!" };
                format!(
                    "{}: {}{}{}",
                    arg.node.name,
                    arg.node.ty.node.base,
                    nullable,
                    print_default_value(arg.node.default_value.as_ref())
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        format!("({})", args)
    } else {
        String::new()
    };
    let doc = field.description.as_ref().map_or(String::new(), |d| {
        format!(r#"  """{}  {}{}  """{}"#, "\n", d.node, "\n", "\n")
    });
    let node = &format!(
        "  {}{}: {} {}",
        field.name.node, args_str, field.ty.node, directives
    );
    doc + node.trim_end()
}

fn print_default_value(value: Option<&Positioned<ConstValue>>) -> String {
    value
        .as_ref()
        .map(|val| format!(" = {val}"))
        .unwrap_or_default()
}

fn print_input_value(field: &async_graphql::parser::types::InputValueDefinition) -> String {
    let directives_str = print_pos_directives(&field.directives);
    let doc = get_formatted_docs(field.description.as_ref().map(|d| d.node.clone()), 2);
    format!(
        "{}  {}: {}{}{}",
        doc,
        field.name.node,
        field.ty.node,
        directives_str,
        print_default_value(field.default_value.as_ref())
    )
}
fn print_directive(directive: &DirectiveDefinition) -> String {
    let args = directive
        .arguments
        .iter()
        .map(|arg| format!("{}: {}", arg.node.name.node, arg.node.ty.node))
        .collect::<Vec<String>>()
        .join(", ");

    if args.is_empty() {
        format!("@{}", directive.name.node)
    } else {
        format!("@{}({})", directive.name.node, args)
    }
}

fn print_directive_type_def(directive: &DirectiveDefinition) -> String {
    let args = directive
        .arguments
        .iter()
        .map(|arg| {
            let doc = get_formatted_docs(arg.node.description.as_ref().map(|d| d.node.clone()), 2);
            format!("{}  {}: {}", doc, arg.node.name.node, arg.node.ty.node)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let doc = get_formatted_docs(directive.description.as_ref().map(|d| d.node.clone()), 0);
    let locations = directive
        .locations
        .iter()
        .map(|d| tailcall_typedefs_common::directive_definition::from_directive_location(d.node))
        .collect::<Vec<_>>();
    let repeatable = if directive.is_repeatable {
        " repeatable"
    } else {
        ""
    };
    let locations = if locations.is_empty() {
        String::new()
    } else {
        format!(" on {}", locations.join(" | "))
    };
    if args.is_empty() {
        format!(
            "{}directive @{}{}{}\n",
            doc, directive.name.node, repeatable, locations
        )
    } else {
        format!(
            "{}directive @{}(\n{}\n){}{}\n",
            doc, directive.name.node, args, repeatable, locations
        )
    }
}

pub fn print(sd: ServiceDocument) -> String {
    // Separate the definitions by type
    let definitions_len = sd.definitions.len();
    let mut schemas = Vec::with_capacity(definitions_len);
    let mut scalars = Vec::with_capacity(definitions_len);
    let mut interfaces = Vec::with_capacity(definitions_len);
    let mut objects = Vec::with_capacity(definitions_len);
    let mut enums = Vec::with_capacity(definitions_len);
    let mut unions = Vec::with_capacity(definitions_len);
    let mut inputs = Vec::with_capacity(definitions_len);
    let mut directives = Vec::with_capacity(definitions_len);

    for def in sd.definitions.iter() {
        match def {
            TypeSystemDefinition::Schema(schema) => schemas.push(print_schema(&schema.node)),
            TypeSystemDefinition::Type(type_def) => match &type_def.node.kind {
                TypeKind::Scalar => scalars.push(print_type_def(&type_def.node)),
                TypeKind::Interface(_) => interfaces.push(print_type_def(&type_def.node)),
                TypeKind::Enum(_) => enums.push(print_type_def(&type_def.node)),
                TypeKind::Object(_) => objects.push(print_type_def(&type_def.node)),
                TypeKind::Union(_) => unions.push(print_type_def(&type_def.node)),
                TypeKind::InputObject(_) => inputs.push(print_type_def(&type_def.node)),
            },
            TypeSystemDefinition::Directive(type_def) => {
                directives.push(print_directive_type_def(&type_def.node))
            }
        }
    }

    // Concatenate the definitions in the desired order
    let sdl_string = schemas
        .into_iter()
        .chain(directives)
        .chain(scalars)
        .chain(inputs)
        .chain(interfaces)
        .chain(unions)
        .chain(enums)
        .chain(objects)
        // Chain other types as needed...
        .collect::<Vec<String>>()
        .join("\n");

    sdl_string.trim_end_matches('\n').to_string()
}
