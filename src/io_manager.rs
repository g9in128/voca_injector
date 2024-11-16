use std::io::Write;

use termcolor::{BufferedStandardStream, Color, ColorSpec, WriteColor};

struct IoManager {
    main_color: ColorSpec,
    sub_color: ColorSpec,
}

impl IoManager {
    pub fn new() -> IoManager {
        let mut manager = IoManager {
            main_color: ColorSpec::new(),
            sub_color: ColorSpec::new(),
        };

        manager.main_color.set_fg(Some(Color::White));
        manager.sub_color.set_fg(Some(Color::Cyan));

        manager
    }

    pub fn first_menu(&self) -> u8 {
        let mut stdout = BufferedStandardStream::stdout(termcolor::ColorChoice::Auto);
        stdout.set_color(&self.main_color).unwrap();
        writeln!(
            &mut stdout,
            "■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■\n■"
        )
        .unwrap();
        stdout.set_color(&self.sub_color).unwrap();
        0
    }
}
