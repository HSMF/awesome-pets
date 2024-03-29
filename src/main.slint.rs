use std::{
    path::Path,
    time::Duration,
};

use slint::{Image, SharedString};

fn main() -> anyhow::Result<()> {
    let main_window = MainWindow::new()?;
    main_window.set_window_name(SharedString::from(env!("CARGO_PKG_NAME")));

    let mw = main_window.as_weak();

    std::thread::spawn(move || {
        let mut i = 0;
        loop {
            mw.upgrade_in_event_loop(move |win| {
                let image =
                    Image::load_from_path(Path::new(&format!("icons/dog/idle/{i}.png"))).unwrap();
                win.set_hey(image);
            })
            .unwrap();
            std::thread::sleep(Duration::from_millis(190));
            i += 1;
            i %= 8;
        }
    });

    main_window.run()?;

    Ok(())
}

slint::slint! {
    component Dawg inherits Rectangle {
        in-out property <image> icon;
        width: 128px;
        height: 88px;
        Image {
            source: icon;
            width: parent.width;
            height: parent.height;
        }

    }

    export component MainWindow inherits Window {
        in property <image> hey;
        in property <string> window_name;
        title: window_name;
        width: 128px;
        height: 88px;
        callback foo();
        background: #00000000;
        Dawg{
            icon: hey;
        }
    }
}
