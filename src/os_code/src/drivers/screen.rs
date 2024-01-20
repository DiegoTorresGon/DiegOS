use core::arch::asm;
use core::default::Default;
use core::fmt;

//Available collors to use with VGA-FrameBugger:
//  Black   -   0,  Red     -   4,  Dark Grey   -   8,  Light Red       -   12,
//  Blue    -   1,  Magenta -   5,  Light Blue  -   9,  Ligth Magenta   -   13,
//  Green   -   2,  Brown   -   6,  Light Green -  10,  Light Brown     -   14,
//  Cyan    -   3,  Light   -   7,  Light Cyan  -  11,  White           -   15, 
//                  Grey
pub const FB_BLACK : u8 = 0; pub const FB_RED : u8 = 4; 
pub const FB_DARK_GREY : u8 = 8; pub const FB_LIGHT_RED : u8 = 12;
pub const FB_BLUE : u8 = 1; pub const FB_MAGENTA : u8 = 5;
pub const FB_LIGHT_BLUE : u8 = 9; pub const FB_LIGHT_MAGENTA : u8 = 13;
pub const FB_GREEN : u8 = 2; pub const FB_BROWN : u8 = 6;
pub const FB_LIGHT_GREEN : u8 = 10; pub const FB_LIGHT_BROWN : u8 = 14;
pub const FB_CYAN : u8 = 3; pub const FB_LIGHT_GREY : u8 = 7;
pub const FB_LIGHT_CYAN : u8 = 11; pub const FB_WHITE : u8 = 15;

const FB_COMMAND_PORT : u16 = 0x3d4;
const FB_DATA_PORT : u16 = 0x3d5;
const FB_SEND_HIGH : u8 = 14;
const FB_SEND_LOW : u8 = 15;

const FB_COLS : usize = 80;
const FB_ROWS : usize = 25;

#[derive(Clone)]
pub struct WriteOut {
    rep_code : RepCode,
    pub frame_buff : FrameBuffer,
}

#[derive(Clone)]
pub struct FrameBuffer {
    cursor : ScreenPointer,
    buffer_start : *mut ScreenChar, 
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenChar {
    ascii_char : u8,
    rep_code : RepCode,
}

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct ScreenPointer(usize);

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RepCode(u8);

impl RepCode {
    pub fn new(foreground : u8, background : u8) -> RepCode {
        RepCode((foreground << 4) | background)
    }
}

impl ScreenPointer {
    pub fn from_xy(column_pos : usize, row : usize) -> ScreenPointer {
        ScreenPointer(row * FB_COLS + column_pos)
    }

    pub fn col(&self) -> usize {
        (self.0 % FB_COLS) as usize
    }

    pub fn row(&self) -> usize {
        (self.0 / FB_COLS) as usize
    }

    fn inc(&mut self) {
        self.0 += 1;
    }

    //Will cause undefined behaviour
    //cursor is in first row.
    fn dec_row(&mut self) {
        self.0 -= FB_COLS;
    }

    fn is_out(&self) -> bool {
        self.0 >= FrameBuffer::SIZE
    }

}


impl ScreenChar {
    pub fn new(ascii_char : u8, foreground : u8, background : u8) -> ScreenChar {
        ScreenChar {
            ascii_char,
            rep_code : RepCode::new(foreground, background),
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> FrameBuffer {
        FrameBuffer::new(ScreenPointer::default())
    }
}

impl FrameBuffer {
    pub const SIZE : usize = (FB_COLS * FB_ROWS) as usize;
    pub const VGA_BUFFER : *mut ScreenChar = 0xb8000 as *mut ScreenChar;

    pub fn new(cursor : ScreenPointer) -> FrameBuffer {
        FrameBuffer {
            cursor,
            buffer_start : Self::VGA_BUFFER.cast(),
        }
    }

    pub fn inc_cursor(&mut self) {
        self.cursor.inc();
        if !self.cursor.is_out() {
            self.move_cursor(self.cursor.0 as u16);
        }
    }

    pub fn write_buff(&mut self, write_buffer : &[u8], rep_code : RepCode) {
        if self.cursor.is_out() { self.scroll(); }

        for ascii_char in write_buffer.iter() {
            unsafe {
                *self.buffer_start.wrapping_add(self.cursor.0)  
                    = ScreenChar{ ascii_char : *ascii_char, rep_code };
            }
            self.inc_cursor();
        }
    }

    pub fn write_times(&mut self, byte : &u8, rep_code : RepCode, times : usize) {
        for _ in 0..times {
            if self.cursor.is_out() { self.scroll() }
            unsafe {
                *self.buffer_start.wrapping_add(self.cursor.0)
                    = ScreenChar{ ascii_char : *byte, rep_code };
            }
            self.inc_cursor();
        }

    }

    pub fn scroll(&mut self) {
        for i in 0..FrameBuffer::SIZE {
            if i >= ScreenPointer::from_xy(FB_COLS, FB_ROWS - 1).0 {
                break;
            }
            unsafe { 
                let to_copy = ScreenPointer(i + FB_COLS);
                *self.buffer_start.wrapping_add(i) = 
                    *self.buffer_start.wrapping_add(to_copy.0);
            }
        }
        self.cursor.dec_row();
    }

    pub fn move_cursor(&mut self, pos : u16) {
        self.cursor = ScreenPointer(pos.into());
        if self.cursor.is_out() { self.scroll(); }

        outb(FB_SEND_HIGH, FB_COMMAND_PORT);
        outb(((pos >> 8) & 0x00ff) as u8, FB_DATA_PORT);
        outb(FB_SEND_LOW, FB_COMMAND_PORT);
        outb((pos & 0x00ff) as u8, FB_DATA_PORT);
    }
}

impl WriteOut {
    const UNPRINTABLE_SUBSTITUTE : u8 = 0xfe;

    pub fn new(frame_buff : FrameBuffer, rep_code : RepCode) -> WriteOut {
        WriteOut {
            rep_code, 
            frame_buff, 
        }
    }

    pub fn write(&mut self, out_str : &str) {
        for ascii_char in out_str.as_bytes() {
            match ascii_char {
                0x20..=0x7e | b'\n' => self.write_byte(ascii_char),
                _ => self.write_byte(&Self::UNPRINTABLE_SUBSTITUTE),
            }
        }
    }

    fn write_byte(&mut self, byte : &u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                self.frame_buff.write_buff(&[*byte], self.rep_code);
            },
        }
    }

    pub fn clear_screen(&mut self) {
        self.frame_buff.move_cursor(0);
        self.frame_buff.write_times(&0x20, RepCode::new(FB_BLACK, FB_BLACK), 
                                    FrameBuffer::SIZE);
        self.frame_buff.move_cursor(0);
    }

    fn new_line(&mut self) {
        self.frame_buff.move_cursor(
            ScreenPointer::from_xy(0, self.frame_buff.cursor.row() + 1).0 as u16);
    }

}

impl fmt::Write for WriteOut {
    fn write_str(&mut self, s : &str) -> Result<(), fmt::Error> {
        if s.len() > FrameBuffer::SIZE {
            return Err(fmt::Error);
        }
        _ = self.write(s);
        Ok(())
    }
}


fn outb(data : u8, io_port : u16) {
    unsafe {
        asm!{
            "out dx, al",
            in("al") data,
            in("dx") io_port,
        }
    }
}
