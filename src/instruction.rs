use crate::stack::Stack;

fn i_unreachable(_stack: &mut Stack) {
    panic!("unreachable");
}

fn i_nop(_stack: &mut Stack) {
}

fn i_loop(stack: &mut Stack) {
    
}
