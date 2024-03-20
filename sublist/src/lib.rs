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

        if !is_subsequence(list1, list2) {

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
fn check_order_form_list <T: PartialEq+ ToString>(list1: &[T], list2: &[T]) -> bool {
    let arr: Vec<String> = arrayter(list1);
    let arr2 = arrayter(list2);

    let mut positions: Vec<u32>  =Vec::new();
    for (i, val) in arr.iter().enumerate(){
        let position  = arr2.iter().position(|x| x == val).unwrap();
        let is = &arr[i];

        positions.push(position as u32);
    
        println!("{}",  position );

    }
    let mut index: u32 = positions[0];
    let mut boolean = true;
    let _ = positions.iter().map(|c: &u32| {

        if index == 0 {
         return   index = c.to_be();
        }
        if(index ==1) {
            return index = c - index;
        }
        let var_name = boolean == false;
        // return var_name;
    });

    return boolean
 }


 pub fn is_subsequence<T: PartialEq + ToString>(list1: &[T], list2: &[T]) -> bool {
    let stringified = arrayter(list1).concat();
    let stringified2 = arrayter(list2).concat();

    let boolean  = stringified2.contains(&stringified);

    println!("d{} d{} d{}", stringified , stringified2 , boolean);

    return boolean

   
}