/*
* @Author: otae
* @Date:   2017-07-20 21:05:47
* @Last Modified by:   otae
* @Last Modified time: 2017-07-25 01:32:28
*/

use color::*;
use float3::*;

pub struct FrameBuffer {
	pub frame: Vec<Vec<char>>,
	pub depth: Vec<Vec<f32>>,
	pub colors: Vec<Vec<Color>>
}

impl FrameBuffer {

	pub fn allocate(&mut self, width: i32, height: i32) {
		for i in 0 .. height {
			let mut line: Vec<char> = Vec::new();
			let mut dline: Vec<f32> = Vec::new();
			let mut cline: Vec<Color> = Vec::new();
			for j in 0 .. width {
					line.push('#');
					dline.push(1000.0);
					cline.push(Color {color: 7});
			}
			self.colors.push(cline);
			self.depth.push(dline);
			self.frame.push(line);
		}
	}

	pub fn clear(&mut self) {
		for j in 0 .. self.frame.len() {
			for i in 0 .. self.frame[0].len () {
				self.frame[i][j] = ' ';
				self.colors[i][j] = Color{color: 5};
				self.depth[i][j] = 1000.0;
			}
		}
	}
}
