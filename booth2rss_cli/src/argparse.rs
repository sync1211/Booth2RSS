use std::env::Args;

pub struct CmdParams {
    pub url: String,
    pub unblur_nsfw: bool,
    pub allow_nsfw: bool,
    pub include_unavailable: bool,
    pub max_pages: u32,
    pub vrc_only: bool,
    pub convert_currency: bool,
    pub convert_target: String
}

pub fn parse_arguments(args: Args) -> Option<CmdParams> {
    let mut cmd_params = CmdParams {
        url: String::new(),
        unblur_nsfw: false,
        allow_nsfw: false,
        include_unavailable: false,
        max_pages: 100,
        vrc_only: false,
        convert_currency: false,
        convert_target: String::new()
    };

    for arg in args.skip(1) {
        match arg.as_str() {
            "--unblur-nsfw" => cmd_params.unblur_nsfw = true,
            "--allow-nsfw" => cmd_params.allow_nsfw = true,
            "--include-unavailable" => cmd_params.include_unavailable = true,
            "--vrc-only" => cmd_params.vrc_only = true,
            _ => {
                if arg.starts_with("https://") {
                    cmd_params.url = arg.to_string();
                } else if let Some(max_pages_str) = arg.strip_prefix("--max-pages=") {
                    let max_pages = match max_pages_str.parse::<u32>() {
                        Ok(max) => max,
                        Err(e) => {
                            log::error!("Unable to parse value for --max-pages '{max_pages_str}' as i32: {e}");
                            return None;
                        }
                    };

                    if max_pages == 0 {
                        log::error!("Value for --max-pages may not be ≤0!");
                        return None;
                    }

                    cmd_params.max_pages = max_pages;
                } else if let Some(target) = arg.strip_prefix("--convert-currency=") {
                    cmd_params.convert_currency = true;
                    cmd_params.convert_target = target.to_string();
                } else {
                    log::error!("Unknown parameter '{arg}'");
                    return None;
                }
            }
        }
    }

    if cmd_params.url.is_empty() {
        log::error!("No URL provided!");
        return None;
    }

    return Some(cmd_params);
}
