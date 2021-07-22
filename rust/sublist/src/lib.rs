#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        return if is_sublist_or_equal(_first_list, _second_list) {
            Comparison::Equal
        } else {
            Comparison::Unequal
        };
    } else if _first_list.len() < _second_list.len() {
        return if is_sublist_or_equal(_first_list, _second_list) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        };
    } else {
        return if is_sublist_or_equal(_second_list, _first_list) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        };
    }
}

fn is_sublist_or_equal<T: PartialEq>(short_list: &[T], long_list: &[T]) -> bool {
    if short_list.len() == 0 {
        return true;
    }
    long_list
        .windows(short_list.len())
        .any(|sublist| short_list.eq(sublist))
}
