#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    if _first_list == _second_list {
        return Comparison::Equal;
    }

    // buscar otra forma de hacerlo menos hardcodeado
    if _first_list.is_empty(){
        return Comparison::Sublist;
    }

    if _second_list.is_empty(){
        return Comparison::Superlist;
    }
     
    for sl in _first_list.windows(_second_list.len()){
        if _second_list == sl {
            return Comparison::Superlist;
        }
    }

    for sl in _second_list.windows(_first_list.len()){
        if _first_list == sl {
            return Comparison::Sublist;
        }
    }
    return Comparison::Unequal;

}



