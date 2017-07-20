/*
* @Author: otae
* @Date:   2017-07-20 16:17:26
* @Last Modified by:   otae
* @Last Modified time: 2017-07-20 22:29:34
*/

use graphic_object::*;
use ncurses::*;
use Fragment::*;
use frame_buffer::*;
use std::thread::sleep;
use std::time::*;

pub fn main_loop (mut meshList: &Vec<Mesh>, mut gObjList: &Vec<GraphicObject>) {

	let mut height: i32 = 0;
	let mut width: i32 = 0;
	let mut a = 0;
	loop {

		a = a + 1;
 		getmaxyx(stdscr(), &mut height, &mut width);

		let mut frameBuffer: FrameBuffer = FrameBuffer{frame: Vec::new()};

		frameBuffer.allocate(width, height);

		frameBuffer.frame[a / width as usize][a % width as usize] = '0';
		gObjList.iter().map(|gObj| gObj.to_fragments()).collect::<Vec<Fragments>>();
		for i in 0 .. width {
			for j in 0 .. height {
				let mut c: String = String::new();
				c.push(frameBuffer.frame[j as usize][i as usize]);
				mvprintw(j,i, &c);
			}
		}
		refresh();
		sleep(Duration::from_millis(100));
	}
}
