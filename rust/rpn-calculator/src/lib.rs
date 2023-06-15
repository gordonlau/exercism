#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs{
        match input {
            CalculatorInput::Value(value) => stack.push(*value),
            operator => {

                if let (Some(value1), Some(value2)) = (stack.pop(), stack.pop()){
                    match operator{
                        CalculatorInput::Add => stack.push(value2 + value1),
                        CalculatorInput::Subtract => stack.push(value2 - value1),
                        CalculatorInput::Multiply => stack.push(value2 * value1),
                        CalculatorInput::Divide => stack.push(value2 / value1),
                        _=> return None
                    }
                }else{
                    return None
                }
            },
        }
    }
    if stack.len() > 1{
        None 
    }else{
        stack.get(0).copied()
    }
}
