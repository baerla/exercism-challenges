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
    let _first = ListWrapper {
        list: _first_list,
        len: _first_list.len(),
    };
    let _second = ListWrapper {
        list: _second_list,
        len: _second_list.len(),
    };

    let mut state = Comparison::Equal;
    let min = cmp::min(_first.len, _second.len);

    if _first.len == _second.len {
        // can only be Equal or Unequal
        for i in 0..min {
            match state {
                Comparison::Equal => {
                    if _first.list[i] != _second.list[i] {
                        return Comparison::Unequal;
                    }
                }
                _ => (),
            }
        }
    } else {
        // can only be Sub- or Superlist or Unequal
        let smaller: ListWrapper<T>;
        let bigger: ListWrapper<T>;
        if _first.len > _second.len {
            state = Comparison::Superlist;
            smaller = _second;
            bigger = _first;
        } else {
            state = Comparison::Sublist;
            smaller = _first;
            bigger = _second;
        }
        let mut already_valid = false;
        let mut old_state = state;

        for i in 0..min {
            if !already_valid {
                if smaller.list[i] == bigger.list[i] {
                    already_valid = true;
                }
            } else {
                if smaller.list[i] != bigger.list[i] {
                    state = Comparison::Unequal;
                }
            }
        }
        if state == Comparison::Unequal {
            state = old_state;
            for first in 1..bigger.len - min + 1 {
                already_valid = false;
                let mut j = first;
                let mut i = 0;
                while j < bigger.len && i < min {
                    if !already_valid {
                        if smaller.list[i] == bigger.list[j] {
                            already_valid = true;
                            j += 1;
                        }
                    } else {
                        if smaller.list[i] != bigger.list[j] {
                            old_state = state;
                            state = Comparison::Unequal;
                            already_valid = false;
                        }
                        j += 1;
                    }
                    i += 1;
                }
                if state == Comparison::Unequal && first != bigger.len - min {
                    state = old_state;
                } else {
                    break;
                }
            }
        }
    }
    return state;
}
