use super::layoutdata::LayoutData;
use ratatui::layout::Rect;
use std::collections::VecDeque;

#[derive(Default)]
pub struct TuiState<'a> {
    qd: VecDeque<&'a String>,
    qt: VecDeque<&'a String>,
    pub dline: [String; 3],
    pub tline: [String; 3],
    max_line_len: u16,
    next_que_word_index: u16,
    line_word_index: u8,
    line_ch_index: u16,
    quit: bool,
    // rect_pos: HashMap<u8, (u8, Rect)>,
}

impl<'a> TuiState<'a> {
    pub fn new(data: &'a LayoutData, frame_width: u16) -> TuiState<'a> {
        let x = TuiState::set_max_line_len(frame_width) as usize;
        let line = [
            String::with_capacity(x + 1),
            String::with_capacity(x + 1),
            String::with_capacity(x + 1),
        ];
        let mut app = TuiState {
            qd: VecDeque::with_capacity(x),
            qt: VecDeque::with_capacity(x),
            dline: line.clone(),
            tline: line,
            max_line_len: x as u16,
            next_que_word_index: 0,
            line_word_index: 0,
            line_ch_index: 0,
            quit: false,
        };
        for _ in app.qd.len()..app.qd.capacity() {
            let (a, b) = data.get_random_pair();
            app.qd.push_back(a);
            app.qt.push_back(b);
        }
        for i in 0..app.dline.len() {
            app.set_line(i);
        }
        app
    }

    fn set_line(&mut self, i: usize) {
        let mut cur_line_len = 0u16;
        for j in self.next_que_word_index as usize..self.qd.capacity() {
            if cur_line_len + self.qd[j].len() as u16 <= self.max_line_len {
                cur_line_len += self.qd[j].len() as u16 + 1;
                self.dline[i].push_str(self.qd[j]);
                self.dline[i].push(' ');
                self.tline[i].push_str(self.qt[j]);
                self.tline[i].push(' ');
                self.next_que_word_index += 1;
            } else {
                break;
            }
        }
    }

    #[inline]
    fn set_max_line_len(frame_width: u16) -> u16 {
        (frame_width * 8) / 10
    }

    #[inline]
    pub fn get_max_line_len(&self) -> u16 {
        self.max_line_len
    }

    pub fn sync_on_ascii_press(&mut self) {
        self.line_ch_index += 1;
    }

    pub fn sync_on_space(&mut self) {
        let it = self.dline[0].chars().nth(self.line_ch_index as usize);

        while let Some(i) = it {
            if i != ' ' {
                self.line_ch_index += 1;
            } else {
                break;
            }
        }
    }

    pub fn sync_on_line_end(&mut self, data: &'a LayoutData) {
        // syncing word queue(app) in App
        for _ in 0..=self.line_word_index {
            self.qd.pop_front();
            self.qt.pop_front();
            let (a, b) = data.get_random_pair();
            self.qd.push_back(a);
            self.qt.push_back(b);
        }
        // syncing line strings
        for i in 0..self.dline.len() - 1 {
            self.dline[i] = self.dline[i + 1].to_owned();
            self.tline[i] = self.tline[i + 1].to_owned();
        }
        // syncing last line string
        self.next_que_word_index -= self.line_word_index as u16;
        self.dline[self.dline.len() - 1].clear();
        self.tline[self.tline.len() - 1].clear();
        self.set_line(self.dline.len() - 1);
    }

    pub fn sync_on_resize(&mut self, frame_width: u16) {
        self.max_line_len = TuiState::set_max_line_len(frame_width);
        for i in 0..self.dline.len() {
            self.set_line(i);
        }
        // line_word_index & line_ch_index is not handled
    }

    #[inline]
    pub fn quit(&mut self) {
        self.quit = true;
    }

    #[inline]
    pub fn should_quit(&self) -> bool {
        self.quit
    }
}
