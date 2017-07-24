/*
* @Author: otae
* @Date:   2017-07-20 21:05:47
* @Last Modified by:   otae
* @Last Modified time: 2017-07-23 18:18:21
*/

pub struct FrameBuffer {
	pub frame: Vec<Vec<char>>,
	pub depth: Vec<Vec<f32>>
}

impl FrameBuffer {

	pub fn allocate(&mut self, width: i32, height: i32) {
		for i in 0 .. height {
			let mut line: Vec<char> = Vec::new();
			let mut dline: Vec<f32> = Vec::new();
			for j in 0 .. width {
					line.push(' ');
					dline.push(1000.0);
			}
			self.depth.push(dline);
			self.frame.push(line);
		}
	}
}
