mod llvm;

use llvm;

trait Compilable {
    fn compile(&self) -> llvm::ValueEnum;
}

struct Type {}

enum ValueStore {}

impl Compilable for ValueStore {
    fn compile(&self) -> llvm::AnyValueEnum {
        unimplemented!()
    }
}

struct Value {
    r#type: Type,
    store: ValueStore,
}

impl Compilable for Value {
    fn compile(&self) -> llvm::AnyValueEnum {
        self.store.compile()
    }
}

struct Variable {
    r#type: Type,
}

impl Compilable for Variable {
    fn compile(&self) -> llvm::AnyValueEnum {
        unimplemented!()
    }
}

struct Get<'a> {
    variable: &'a Variable,
}

impl Get {
    fn new(variable: &Variable) -> Get {
        Get { variable }
    }

    fn r#type(&self) -> &Type {
        self.r#type
    }
}

impl Compilable for Get {
    fn compile(&self) -> llvm::AnyValueEnum {
        self.variable.compile()
    }
}

struct Set<'a> {
    variable: &'a Variable,
    value: &'a Value,
}

impl Set {
    fn new(variable: &Variable, value: &Value) -> Option<Set> {
        if variable.r#type == value.r#type {
            Some(Set { variable, value })
        } else {
            None
        }
    }

    fn r#type(&self) -> &Type {
        self.r#type
    }
}
