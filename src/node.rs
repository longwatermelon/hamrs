pub enum NodeType {
    VariableDefinition,
    Variable,
    FunctionCall,
    Int,
    Str,
    Compound,
    Noop
}

pub struct Node {
    pub node_type: NodeType,

    // variable definition
    pub variable_definition_name: String,
    pub variable_definition_value: *mut Node,

    // variable
    pub variable_name: String,

    // function
    pub function_call_name: String,
    pub function_call_args: Vec<*mut Node>,
   
    // string
    pub string_value: String,

    // int
    pub int_value: i32,

    // compound
    pub compound_value: Vec<*mut Node>
}

pub fn init_node(t: NodeType) -> *mut Node {
    return Box::into_raw(Box::new(Node{
        node_type: t,
        variable_definition_name: String::new(),
        variable_definition_value: 0 as *mut Node,
        variable_name: String::new(),
        function_call_name: String::new(),
        function_call_args: Vec::new(),
        string_value: String::new(),
        int_value: 0,
        compound_value: Vec::new()
    }));
}

