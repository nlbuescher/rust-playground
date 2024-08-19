mod linked_list;

use crate::linked_list::LinkedList;

fn main() {
	let mut list = LinkedList::new();
	list.push_front(1);
	list.push_back(2);
	list.insert(3, 2);
	let v = list.remove(1);

	println!("{v}");

	println!("{list:?} {}", list.len());

	println!("front: {}", list.front().unwrap());
	*list.front_mut().unwrap() = 4;
	println!("front: {}", list.front().unwrap());

	println!("back:  {}", list.back().unwrap());
	*list.back_mut().unwrap() = 5;
	println!("back:  {}", list.back().unwrap());

	println!("{list:?}");
}
