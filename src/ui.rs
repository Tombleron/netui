use console_engine::screen::Screen;

pub struct ListItem {
    pub elements: Vec<String>,
}

pub struct List {
    pub screen: Screen,
    cur_selected: usize,
    offset: usize,
    items: Vec<ListItem>,
    pub spacing: Vec<f32>,
    columns: usize,
}

impl List {
    pub fn new(columns: usize, screen: Screen) -> Self {
        if columns == 0 {
            panic!("Just no");
        }

        Self {
            offset: 0,
            cur_selected: 0,
            items: Vec::new(),
            spacing: vec![0.0; columns],
            columns,
            screen,
        }
    }

    #[allow(dead_code)]
    pub fn auto_spacing(&mut self) {
        for i in 0..self.columns {
            self.spacing.push(1.0 / self.columns as f32 * i as f32);
        }
    }

    pub fn auto_space_rest(&mut self, rest: usize) {
        let remainder = 1.0 - self.spacing[self.columns - rest];

        for x in (self.columns - rest)..self.columns {
            self.spacing[x] = self.spacing[x - 1] + remainder / rest as f32;
        }
    }

    pub fn add_new(&mut self, t: ListItem) {
        self.items.push(t);
    }

    pub fn select_next(&mut self) {
        if self.items.len() == 0 {
            return;
        }

        self.cur_selected = std::cmp::min(self.cur_selected + 1, self.items.len() - 1);

        if self.cur_selected > self.offset + self.screen.get_height() as usize - 1 {
            self.offset += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.cur_selected != 0 {
            self.cur_selected -= 1;
        }

        if self.cur_selected < self.offset {
            self.offset -= 1;
        }
    }

    fn get_beginning(&self, index: usize, x: u32) -> i32 {
        (self.spacing[index] * x as f32) as i32
    }

    pub fn select_end(&mut self) {
        if self.items.len() != 0 {
            self.cur_selected = self.items.len() - 1;

            self.offset = if self.items.len() < self.screen.get_height() as usize {
                0
            } else {
                self.items.len() - self.screen.get_height() as usize
            }
        }
    }

    pub fn select_start(&mut self) {
        self.cur_selected = 0;
        self.offset = 0;
    }

    pub fn resize(&mut self, x: u16, y: u16) {
        let new_x = x as u32 - 2;
        let new_y = y as u32 - 2;

        self.screen = Screen::new(new_x, new_y);

        if self.cur_selected - self.offset > new_y as usize - 1 {
            self.cur_selected = self.offset + new_y as usize - 1;
        }
    }

    pub fn draw(&mut self) {
        let (x, y) = (self.screen.get_width(), self.screen.get_height());
        self.screen.clear();

        if self.items.len() == 0 {
            self.screen.print(x as i32 / 2, y as i32 / 2, "Empty");
            return;
        }

        // Takes slice of elements to render
        let slice = &self.items
            [self.offset..std::cmp::min(self.items.len(), self.offset + self.items.len())];

        for (i, item) in slice.iter().enumerate() {
            let mut items = item.elements.iter();

            for j in 0..self.columns {
                let start_x = self.get_beginning(j, x);
                
                if let Some(text) = items.next() {
                    if i + self.offset == self.cur_selected {
                        self.screen.line(
                            start_x,
                            i as i32,
                            self.screen.get_width() as i32,
                            i as i32,
                            console_engine::pixel::pxl_bg(' ', console_engine::Color::White),
                        );
                        self.screen.print_fbg(
                            start_x,
                            i as i32,
                            text,
                            console_engine::Color::Black,
                            console_engine::Color::White,
                        );

                    } else {
                        self.screen.print(start_x, i as i32, text);
                    }
                } else {
                    self.screen.print(start_x, i as i32, "-");
                }
            }
        }
    }
}
