use std::env::Args;

pub struct CmdParams {
    pub url: String,
    pub unblur_nsfw: bool,
    pub allow_nsfw: bool,
    pub include_unavailable: bool,
    pub max_pages: i32,
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

    for arg in args.into_iter() {
        match arg.as_str() {
            "--unblur-nsfw" => cmd_params.unblur_nsfw = true,
            "--allow-nsfw" => cmd_params.allow_nsfw = true,
            "--include-unavailable" => cmd_params.include_unavailable = true,
            "--vrc-only" => cmd_params.vrc_only = true,
            _ => {
                if arg.starts_with("https://") {
                    cmd_params.url = arg.to_string();
                } else if let Some(max_pages_str) = arg.strip_prefix("--max-pages=") {
                    cmd_params.max_pages = match max_pages_str.parse::<i32>() {
                        Ok(max) => max,
                        Err(e) => {
                            println!("ERROR: Unable to parse '{max_pages_str}' as i32: {e}");
                            return None;
                        }
                    }
                } else if let Some(target) = arg.strip_prefix("--convert-currency=") {
                    cmd_params.convert_currency = true;
                    cmd_params.convert_target = target.to_string();
                } else {
                    println!("WARNING: Unknown parameter '{arg}'");
                }
            }
        }
    }

    if cmd_params.url.is_empty() {
        println!("ERROR: No URL provided!");
        return None;
    }

    return Some(cmd_params);
}
