/*
* @Author: otae
* @Date:   2017-07-20 21:05:47
* @Last Modified by:   otae
* @Last Modified time: 2017-07-20 22:25:50
*/

pub struct FrameBuffer {
	pub frame: Vec<Vec<char>>,
}

impl FrameBuffer {

	pub fn allocate(&mut self, width: i32, height: i32) {
		for i in 0 .. height {
			let mut line: Vec<char> = Vec::with_capacity(width as usize);
			for j in 0 .. width {
				line.push('*');	
			}
			self.frame.push(line);

		}
	}
}
