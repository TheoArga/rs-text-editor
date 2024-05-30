use fltk::{enums::*, menu::SysMenuBar, prelude::*, utils::oncelock::Lazy, *};
use std::path::PathBuf;

use crate::rope::Rope;

const DEFAULT_WIDTH: i32 = 720;
const DEFAULT_HEIGHT: i32 = 480;
static STATE: Lazy<app::GlobalState<State>> = Lazy::new(app::GlobalState::<State>::get);

pub struct State {
    pub saved: bool,
    pub curFile: PathBuf,
}

impl State {
    fn new(rope: Rope) -> Self {
        State {
            saved: true,
            curFile: PathBuf::new(),
        }
    }
}

pub fn init() {
    let app = app::App::default();
    let screen_bb: (i32, i32, i32, i32) = fltk::app::screen_xywh(0);
    let app_x = (screen_bb.2 / 2) - (DEFAULT_WIDTH / 2);
    let app_y = (screen_bb.3 / 2) - (DEFAULT_HEIGHT / 2);
    let mut window = window::Window::default()
        .with_pos(app_x, app_y)
        .with_size(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    window.set_xclass("ted");
    {
        let mut column = group::Flex::default_fill().column();
        column.set_pad(0);
        let mut menu = menu::SysMenuBar::default();
        init_sysmenu(&mut menu);
    }
    window.end();
    window.show();
    app.run().unwrap();
}

fn init_sysmenu(menu: &mut menu::SysMenuBar) {
    menu.add(
        "&File/new...\t",
        Shortcut::Ctrl | Shortcut::Alt | 'l',
        menu::MenuFlag::Normal,
        menu_callback,
    );
    menu.add(
        "&File/Open...\t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        menu_callback,
    );
    menu.add(
        "&File/Save\t",
        Shortcut::Ctrl | 's',
        menu::MenuFlag::Normal,
        menu_callback,
    );
    menu.add(
        "&File/Save as...\t",
        Shortcut::Ctrl | Shortcut::Shift | 's',
        menu::MenuFlag::Normal,
        menu_callback,
    );
    let quitbtn = menu.add(
        "&File/QUIT\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_callback,
    );
    menu.at(quitbtn)
        .unwrap()
        .set_label_color(Color::Red.darker().darker());
    menu.add(
        "&Edit/Copy\t",
        Shortcut::Ctrl | 'c',
        menu::MenuFlag::Normal,
        menu_callback,
    );
    menu.add(
        "&Edit/Paste\t",
        Shortcut::Ctrl | 'v',
        menu::MenuFlag::Normal,
        menu_callback,
    );
    menu.add(
        "&Edit/Cut\t",
        Shortcut::Ctrl | 'x',
        menu::MenuFlag::Normal,
        menu_callback,
    );
    menu.add(
        "&Selection/Select All\t",
        Shortcut::Ctrl | 'a',
        menu::MenuFlag::Normal,
        menu_callback,
    );
    menu.add(
        "&Help/About...\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_callback,
    );
}

fn menu_callback(menu: &mut impl MenuExt) {}

fn quit_cb() {
    STATE.with(|s| {
        if s.saved {
            app::quit();
        } else {
            let c = dialog::choice2_default(
                "Are you sure you want to exit without saving?",
                "Yes",
                "No",
                "",
            );
            if c == Some(0) {
                app::quit();
            }
        }
    });
}
