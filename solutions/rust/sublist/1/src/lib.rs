#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let mut result = Comparison::Unequal;
    if first_list == second_list {
        result = Comparison::Equal;
    } else if first_list.len() > second_list.len() {
        if second_list.len() == 0
            || first_list
                .windows(second_list.len())
                .any(|x| x == second_list)
        {
            result = Comparison::Superlist;
        }
    } else if second_list.len() > first_list.len() {
        if first_list.len() == 0
            || second_list
                .windows(first_list.len())
                .any(|x| x == first_list)
        {
            result = Comparison::Sublist
        }
    }
    result
}
