use std::vec;
use naulang::objectspace::primitives::{Object};

#[derive(Clone)]
pub struct Frame {
	stack: vec::Vec<Object>,
	stack_pointer: u32,
	previous_frame: Option<Box<Frame>>,
	access_link: Option<Box<Frame>>,
	pc: u32,
	locals: vec::Vec<Object>,
}

impl Frame {
	pub fn new(stack_size: u32, local_count: u32) -> Box<Frame> {
		let mut new_frame = Frame {
			stack: vec::Vec::new(),
			stack_pointer: 0,
			pc: 0,
			access_link: Option::None,
			previous_frame: Option::None,
			locals: vec::Vec::new()
		};

		for x in 0..local_count {
			new_frame.locals.push(Object::None);
		}

		return Box::new(new_frame);
	}

	pub fn pop(&mut self) -> Option<Object> {
		self.stack.pop()
	}

	pub fn push(&mut self, object: Object) -> () {
		self.stack.push(object);
	}

	pub fn set_local_at(&mut self, index: usize, object: Object) -> () {
		self.locals[index] = object;
	}

	pub fn get_local_at(&self, index: usize) -> Option<Object> {
		if index >= self.locals.len() || index < 0 {
			Option::None
		} else {
			Option::Some(self.locals[index].clone())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Frame;
	use naulang::objectspace::primitives::{Object,IntegerObject};

	#[test]
	fn test_set_get_local_at() {
		let mut frame = Frame::new(3, 1);
		let integer_object = IntegerObject::new(42);
		frame.set_local_at(0, Object::Integer(integer_object));
		let local = frame.get_local_at(0);

		let internal_value = match local {
			Some(object) => match object {
				Object::Integer(i_obj) => i_obj.get_value(),
				_ => 0,
			},
			None => 0
		};

		assert!(internal_value == 42);
	}

	#[test]
	fn test_push_pop() {
		let mut frame = Frame::new(3, 1);
		let integer_object = IntegerObject::new(42);
		frame.push(Object::Integer(integer_object));
		let popped_object = frame.pop();

		let internal_value = match popped_object {
			Some(object) => match object {
				Object::Integer(i_obj) => i_obj.get_value(),
				_ => 0,
			},
			None => 0
		};

		assert!(internal_value == 42);

	}
}
