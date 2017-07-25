/*
* @Author: otae
* @Date:   2017-07-24 22:25:31
* @Last Modified by:   otae
* @Last Modified time: 2017-07-25 01:57:18
*/

use ncurses::*;
use float3::*;

pub struct Color {
	pub color: i16
}
pub fn init() {
	initscr();
	start_color();
	 init_color(COLOR_BLACK, 0, 0, 0);
	 // init_pair(7, COLOR_WHITE, COLOR_BLACK);
	 // init_pair(1, COLOR_RED, COLOR_BLACK);
	 // init_pair(2, COLOR_BLUE, COLOR_BLACK);
	 // init_pair(3, COLOR_YELLOW, COLOR_BLACK);
	 // init_pair(4, COLOR_BLACK, COLOR_BLACK);
	 // init_pair(5, COLOR_GREEN, COLOR_BLACK);
	 // init_pair(6, COLOR_BLACK, COLOR_BLACK);

	 init_pair(7, COLOR_WHITE, COLOR_WHITE);
	 init_pair(1, COLOR_RED, COLOR_RED);
	 init_pair(2, COLOR_BLUE, COLOR_BLUE);
	 init_pair(3, COLOR_YELLOW, COLOR_YELLOW);
	 init_pair(4, COLOR_BLACK, COLOR_BLACK);
	 init_pair(5, COLOR_GREEN, COLOR_GREEN);
	 init_pair(6, COLOR_BLACK, COLOR_BLACK);
}

impl Color {
	pub fn apply(&self) {
		attron(COLOR_PAIR(self.color));
	}
}
