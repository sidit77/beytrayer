use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, TranslateMessage};
use betrayer::{Menu, MenuItem, TrayIconBuilder};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Signal {
    Profile(u32),
    Open,
    Quit
}

fn main() {
    let tray = TrayIconBuilder::new()
        .with_menu(Menu::new([
            MenuItem::menu("Profiles", [
                MenuItem::button("Music", Signal::Profile(0)),
                MenuItem::button("Gaming", Signal::Profile(1)),
            ]),
            MenuItem::separator(),
            MenuItem::button("Open", Signal::Open),
            MenuItem::button("Quit", Signal::Quit)
        ]))
        .build(|s| println!("Clicked: {:?}", s));

    unsafe {
        let mut msg: MSG = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}