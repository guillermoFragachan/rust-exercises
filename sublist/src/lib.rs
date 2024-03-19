use std::fmt::{Debug};
// use num_traits::Num;

#[derive(PartialEq, Eq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + ToString >(list1: &[T], list2: &[T]) -> Comparison {

    if list1.len() == list2.len() {

        if !list2.iter().all(|c| list1.contains(c)) {
          
            return Comparison::Unequal;
        }
       return Comparison::Equal
    }
    else if list1.len() < list2.len() {
        println!("{:?} {:?} {} sublist", arrayter(list1), arrayter(list2),list1.iter().all(|c| list2.contains(c))  );

        if !list1.iter().any(|c| list2.contains(c)) {

            return Comparison::Unequal;
        }
        return Comparison::Sublist
    }
    else if list1.len() > list2.len() {
        return Comparison::Superlist
    }


    Comparison::Unequal

}
fn arrayter<T: PartialEq + ToString>(list1: &[T]) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for val in list1 {
        let str = val.to_string();

        vec.push(str);
    }

    return vec;
}

fn calculate_eq<T: PartialEq>(list1: &[T], list2: &[T]) -> bool {
    list1.iter().all(|element| list2.contains(element))
}
fn first_list_contains_all <T: PartialEq>(list1: &[T], list2: &[T]) -> bool {
    list1.iter().any(|c| list2.contains(c)) 
 }


fn return_larger_list<'a, T: PartialEq> (list1: &'a [T], list2: &'a [T]) -> &'a [T] {
    if list1.len() > list2.len() {
        return list1;
    }
    return list2;
}

fn return_smaller_list<'a, T: PartialEq> (list1: &'a [T], list2: &'a [T]) -> &'a [T] {
    if list1.len() < list2.len() {
        return list1;
    }
    return list2;
}