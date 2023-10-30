#[derive(Debug)]
pub enum RenType {
    Integer,
    String,
    Word,
    LitWord,
    SetWord,
    GetWord,
    Block,
}

#[derive(Debug)]
pub enum ValueType {
    Integer     (i32),
    String      (String),
    Word        (String),
    LitWord     (String),
    SetWord     (String),
    GetWord     (String),
    Block       (Vec<Value>),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum WordType {
    Word,
    LitWord,
    SetWord,
    GetWord,
}

#[derive(Debug)]
pub struct Value {
    pub value: ValueType,
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
            RenType::SetWord => {
                ValueType::SetWord(content)
            },
            RenType::GetWord => {
                ValueType::GetWord(content)
            },
            RenType::LitWord => {
                ValueType::LitWord(content)
            },
//          RenType::Block => {
//              ValueType::Block(content)
//          },
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
