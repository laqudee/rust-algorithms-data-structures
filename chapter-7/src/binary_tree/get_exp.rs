use crate::binary_tree::Link;
use std::fmt::{Debug, Display};

// 按照节点位置返回节点组成的字符串
pub fn get_exp<T: Clone + Debug + Display>(bt: Link<T>) -> String {
  let mut exp = "".to_string();
  if !bt.is_none() {
      exp = "(".to_string() + &get_exp(bt.as_ref().unwrap().get_left());
      exp += &bt.as_ref().unwrap().get_key().to_string();
      exp += &(get_exp(bt.as_ref().unwrap().get_right()) + ")");
  }

  exp
}
