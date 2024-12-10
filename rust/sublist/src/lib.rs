use std::cmp;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

#[derive(Copy, Clone, Debug)]
struct ListWrapper<'a, T> {
    list: &'a [T],
    len: usize,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_list = ListWrapper {
        list: _first_list,
        len: _first_list.len(),
    };
    let second_list = ListWrapper {
        list: _second_list,
        len: _second_list.len(),
    };

    if first_list.len == second_list.len {
        if first_list.list == second_list.list {
            return Comparison::Equal;
        }
    } else if first_list.len < second_list.len {
        if is_sublist(first_list, second_list) {
            return Comparison::Sublist;
        }
    } else {
        if is_sublist(second_list, first_list) {
            return Comparison::Superlist;
        }
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(small: ListWrapper<T>, big: ListWrapper<T>) -> bool {
    for i in 0..big.len - small.len + 1 {
        if big.list[i..i + small.len] == *small.list {
            return true;
        }
    }

    false
}
