mod borrowing;
mod condition;
mod constant;
mod data_structure;
mod datatype;
mod for_loop;
mod function;
mod loops;
mod matching;
mod ownership;
mod scope;
mod string;
mod variable;
mod while_loop;

fn main() {
    variable::variables();

    datatype::data_types();
    string::strings();

    scope::scopes();

    constant::constants();

    condition::conditions();

    loops::loops();
    for_loop::for_loop();
    while_loop::while_loop();

    function::function();

    matching::matches();

    ownership::ownership();
    borrowing::borrowing();

    data_structure::data_structure();
}
