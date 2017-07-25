/*
* @Author: otae
* @Date:   2017-07-20 16:17:26
* @Last Modified by:   otae
* @Last Modified time: 2017-07-25 01:33:05
*/

use graphic_object::*;
use ncurses::*;
use Fragment::*;
use frame_buffer::*;
use std::thread::sleep;
use std::time::*;
use mesh::*;
use transform::*;
use camera_controler::*;

pub fn main_loop (mut meshList: &Vec<Mesh>, mut gObjList: &mut Vec<GraphicObject>) {

	let mut height: i32 = 0;
	let mut width: i32 = 0;
	let mut cam: CameraControler = CameraControler::new();

	loop {

		getmaxyx(stdscr(), &mut height, &mut width);

		let mut frameBuffer: FrameBuffer = FrameBuffer{frame: Vec::new(), depth: Vec::new(), colors: Vec::new()};

		frameBuffer.allocate(width, height);
		cam.update();

		for i in 0 .. gObjList.len() {
			for x in gObjList[i].to_fragments(cam.transform) {
				x.rasterize(&mut frameBuffer);
			}
		}
		for i in 0 .. width {
			for j in 0 .. height {
				frameBuffer.colors[j as usize][i as usize].apply();
				let mut c: String = String::new();
				c.push(frameBuffer.frame[j as usize][i as usize]);
				mvprintw(j,i, &c);
				attroff(COLOR_PAIR(1));
			}
		}
		refresh();
	}
}
