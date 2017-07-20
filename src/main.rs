/*
* @Author: otae
* @Date:   2017-07-20 16:17:30
* @Last Modified by:   otae
* @Last Modified time: 2017-07-20 21:06:23
*/
extern crate ncurses;

mod window_loop;
mod graphic_object;
mod float3;
mod Fragment;
mod float2;
mod frame_buffer;

use ncurses::*;
use window_loop::*;
use graphic_object::*;

fn init () {
	initscr();
	keypad(stdscr(), true);
	noecho();
}

fn main() {

	let mut meshList: Vec<Mesh> = Vec::new();
	let mut gObjList: Vec<GraphicObject> = Vec::new();
	
	init();

	main_loop(&meshList, &gObjList);
	endwin();
}

