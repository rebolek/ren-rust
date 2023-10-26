#[derive(Debug)]
pub enum RenType {
    Integer,
    String,
    Word,
    Block,
}

#[derive(Debug)]
pub enum ValueType {
    Integer     (i32),
    String      (String),
    Word        (String),
    Block       (Vec<Value>),
}

#[derive(Debug)]
pub struct Value {
    value: ValueType,
}

impl Value {
    pub fn make(value: ValueType) -> Self {
        Value {
            value,
        }
    }

    pub fn convert(
        ren_type: RenType,
        content: String,
    )   -> Value {
        let val: ValueType = match ren_type {
            RenType::Integer => {
                ValueType::Integer(content.parse::<i32>().unwrap())
            },
            RenType::String => {
                ValueType::String(content)
            },
            RenType::Word => {
                ValueType::Word(content)
            },
            _ => {
                ValueType::String(content)
            },
        };
        Value::make(
            //ren_type.to_string(),
            val,
        )
    }
}
