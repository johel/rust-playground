#![allow(dead_code, unused_variables)]
#[cfg(debug_assertions)]

use std::fmt;
use std::fmt::Debug;


pub enum Comparison {
  Sublist,
  Superlist,
  Equal,
  Unequal
}


impl fmt::Debug for Comparison {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          &Comparison::Sublist => write!(f, "Sublist"),
          &Comparison::Superlist => write!(f, "Superlist"),
          &Comparison::Equal => write!(f, "Equal"),
          _ => write!(f, "Unequal"),
      }
  }
}

pub fn sublist<T>(list: &[T], list2: &[T]) -> Comparison 
  where 
      T: PartialEq,
      T: Debug
{
  let is_sublist = if list.len() <= list2.len() {
      is_sublist(list, list2)
  }else {
      is_sublist(list2, list)
  };

  if !is_sublist {
      Comparison::Unequal
  } else if list.len() == list2.len() {
      Comparison::Equal
  }else if list.len() < list2.len() {
      Comparison::Sublist
  }else {
      Comparison::Superlist
  }
}

fn is_sublist<T>(list: &[T], larger_list: &[T]) -> bool
  where T: PartialEq,
        T: Debug
{
  let diff = larger_list.len() - list.len();
  let mut is_sublist = true;
  let mut cont = 0;
  let mut offset = 0;
  while is_sublist && cont < list.len() {
      if list[cont] != larger_list[cont + offset] {
          offset+=1;
          cont = 0;
          if offset > diff {
              is_sublist = false;
          }
      } else {
          cont+=1;
      }
  }

  return is_sublist;

}
