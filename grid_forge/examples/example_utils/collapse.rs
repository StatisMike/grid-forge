#[derive(Debug)]
pub struct ArgHelper {
    gif: bool,
    debug: bool,
    skip_position: bool,
    skip_entrophy: bool,
}

impl ArgHelper {
    pub const GIF: &'static str = "--gif";
    pub const DEBUG: &'static str = "--debug";
    pub const SKIP_POSITION: &'static str = "--skip-position";
    pub const SKIP_ENTROPHY: &'static str = "--skip-entrophy";

    pub fn gather() -> Self {
        let args = std::env::args().collect::<Vec<_>>();

        let gif = args.contains(&Self::GIF.to_owned());
        let debug = args.contains(&Self::DEBUG.to_owned());
        let skip_position = args.contains(&Self::SKIP_POSITION.to_owned());
        let skip_entrophy = args.contains(&Self::SKIP_ENTROPHY.to_owned());

        if gif && debug {
            panic!("cannot use both `--gif` and `--debug` flags at the same time");
        }

        Self {
            gif,
            debug,
            skip_position,
            skip_entrophy,
        }
    }

    pub fn gif(&self) -> bool {
        self.gif
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn skip_position(&self) -> bool {
        self.skip_position
    }

    pub fn skip_entrophy(&self) -> bool {
        self.skip_entrophy
    }
}