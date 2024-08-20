#![allow(dead_code)]

struct Node<T> {
	next: Option<Box<Node<T>>>,
	value: T,
}

pub struct LinkedList<T> {
	len: usize,
	head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
	pub fn new() -> Self { LinkedList { head: None, len: 0 } }

	pub fn len(&self) -> usize { self.len }

	pub fn append(&mut self, other: &mut LinkedList<T>) {
		self.len += other.len;

		match self.head.as_mut() {
			None => self.head = other.head.take(),
			Some(mut tail) => {
				while tail.next.is_some() {
					tail = tail.next.as_mut().unwrap();
				}
				tail.next = other.head.take();
			}
		}
	}

	pub fn iter(&self) -> Iter<T> {
		Iter { current: self.head.as_deref() }
	}

	pub fn front(&self) -> Option<&T> {
		self.head.as_ref().map(|node| &node.value)
	}

	pub fn front_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node| &mut node.value)
	}

	pub fn back(&self) -> Option<&T> {
		let mut tail = self.head.as_ref()?;
		while tail.next.is_some() {
			tail = tail.next.as_ref()?;
		}
		Some(&tail.value)
	}

	pub fn back_mut(&mut self) -> Option<&mut T> {
		let mut tail = self.head.as_mut()?;
		while tail.next.is_some() {
			tail = tail.next.as_mut()?;
		}
		Some(&mut tail.value)
	}

	pub fn push_front(&mut self, value: T) {
		self.len += 1;
		self.head = Some(Box::new(Node { value, next: self.head.take() }));
	}

	pub fn pop_front(&mut self) -> Option<T> {
		self.head.take().map(|mut node| {
			self.len -= 1;
			self.head = node.next.take();
			node.value
		})
	}

	pub fn push_back(&mut self, value: T) {
		self.len += 1;
		let node = Some(Box::new(Node { value, next: None }));

		match self.head.as_mut() {
			None => self.head = node,
			Some(mut tail) => {
				while tail.next.is_some() {
					tail = tail.next.as_mut().unwrap();
				}
				tail.next = node;
			}
		}
	}

	pub fn pop_back(&mut self) -> Option<T> {
		if self.head.as_ref()?.next.is_none() {
			return self.head.take().map(|node| node.value);
		}

		let mut current = self.head.as_mut()?;
		while current.next.as_ref()?.next.is_some() {
			current = current.next.as_mut()?;
		}
		self.len -= 1;
		current.next.take().map(|node| node.value)
	}

	pub fn split_off(&mut self, at: usize) -> Self {
		assert!(at <= self.len, "Cannot split off at a nonexistent index");

		if at == 0 {
			let len = self.len;
			self.len = 0;
			return LinkedList { len, head: self.head.take() };
		}
		if at == self.len {
			return Self::new();
		}

		let mut i = 0;
		let mut new_tail = self.head.as_mut().unwrap();
		while i < at - 1 {
			new_tail = new_tail.next.as_mut().unwrap();
			i += 1;
		}
		Self { len: self.len - at, head: new_tail.next.take() }
	}

	pub fn insert(&mut self, value: T, at: usize) {
		assert!(at <= self.len, "Cannot insert at a nonexistent index");

		if at == 0 {
			return self.push_front(value);
		}

		let mut i = 0;
		let mut current = self.head.as_mut().unwrap();
		while i < at - 1 {
			current = current.next.as_mut().unwrap();
			i += 1;
		}

		current.next = Some(Box::new(Node { value, next: current.next.take() }));
		self.len += 1;
	}

	pub fn remove(&mut self, at: usize) -> T {
		assert!(at < self.len, "Cannot remove at a nonexistent index");

		if at == 0 {
			return self.pop_front().unwrap();
		}

		let mut i = 0;
		let mut current = self.head.as_mut().unwrap();
		while i < at - 1 {
			current = current.next.as_mut().unwrap();
			i += 1;
		}
		let mut node = current.next.take().unwrap();
		current.next = node.next.take();
		self.len -= 1;
		node.value
	}
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedList<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self).finish()
	}
}

pub struct Iter<'a, T: 'a> {
	current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		self.current.map(|node| {
			self.current = node.next.as_deref();
			&node.value
		})
	}
}

pub struct IntoIter<T> {
	current: Option<Box<Node<T>>>,
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.current.take().map(|node| {
			self.current = node.next;
			node.value
		})
	}
}

impl<T> IntoIterator for LinkedList<T> {
	type Item = T;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter { current: self.head }
	}
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}