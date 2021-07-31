use crate::node;

pub struct Visitor {
    pub variable_defs: Vec<*mut node::Node>
}

pub fn init_visitor() -> Visitor {
    return Visitor{
        variable_defs: Vec::new()
    };
}

pub unsafe fn visit(visitor: &mut Visitor, node: *mut node::Node) -> *mut node::Node {
    if node == 0 as *mut node::Node { return node; }

    match (*node).node_type {
        node::NodeType::VariableDefinition => return visit_variable_definition(visitor, node),
        node::NodeType::Variable => return visit_variable(visitor, node),
        node::NodeType::FunctionCall => return visit_function_call(visitor, node),
        node::NodeType::Str => return node,
        node::NodeType::Int => return node,
        node::NodeType::Compound => return visit_compound(visitor, node)
    }
}

pub fn visit_variable_definition(visitor: &mut Visitor, node: *mut node::Node) -> *mut node::Node {
    visitor.variable_defs.push(node);
    return node;
}

pub unsafe fn visit_variable(visitor: &mut Visitor, node: *mut node::Node) -> *mut node::Node {
    for i in 0..visitor.variable_defs.len() {
        let def = visitor.variable_defs[i];

        if (*def).variable_definition_name == (*node).variable_name {
            return visit(visitor, (*def).variable_definition_value);
        }
    }

    panic!("Undefined variable {}", (*node).variable_name);
}

pub unsafe fn visit_compound(visitor: &mut Visitor, node: *mut node::Node) -> *mut node::Node {
    for i in 0..(*node).compound_value.len() {
        visit(visitor, (*node).compound_value[i]);
    }

    return 0 as *mut node::Node;
}

pub unsafe fn visit_function_call(visitor: &mut Visitor, node: *mut node::Node) -> *mut node::Node {
    if (*node).function_call_name == "pront" {
        return builtin_function_print(visitor, &(*node).function_call_args);
    }

    panic!("Undefined function '{}'", (*node).function_call_name.as_str());
}

unsafe fn builtin_function_print(visitor: &mut Visitor, args: &Vec<*mut node::Node>) -> *mut node::Node {
    for i in 0..args.len() {
        let visited = visit(visitor, args[i]);

        match (*visited).node_type {
            node::NodeType::Str => print!("{} ", (*visited).string_value),
            node::NodeType::Int => print!("{} ", (*visited).int_value),
            _ => print!("Unrecognized type")
        }
    }

    println!("");
    return 0 as *mut node::Node;
}

