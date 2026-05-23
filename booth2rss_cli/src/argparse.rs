use std::env::Args;

pub struct CmdParams {
    pub url: String,
    pub unblur_nsfw: bool,
    pub allow_nsfw: bool,
    pub include_unavailable: bool,
    pub limit: i32,
    pub max_pages: i32,
    pub vrc_only: bool
}

pub fn parse_arguments(args: Args) -> Option<CmdParams> {
   let mut url_res: Option<String> = None;
   let mut unblur_nsfw = false;
   let mut allow_nsfw = false;
   let mut include_unavailable = false;
   let mut limit = 100;
   let mut max_pages = 100;
   let mut vrc_only = false;

    for arg in args.into_iter() {
        match arg.as_str() {
            "--unblur-nsfw" => unblur_nsfw = true,
            "--allow-nsfw" => allow_nsfw = true,
            "--include-unavailable" => include_unavailable = true,
            "--vrc-only" => vrc_only = true,
            //TODO: Limit and max_pages
            _ => {
                if arg.starts_with("https://") {
                    url_res = Some(arg.to_owned());
                } else if let Some(limit_str) = arg.strip_prefix("--limit=") {
                    limit = match limit_str.parse::<i32>() {
                        Ok(limit) => limit,
                        Err(e) => {
                            println!("ERROR: Unable to parse '{limit_str}' as i32: {e}");
                            return None;
                        }
                    }
                } else if let Some(max_pages_str) = arg.strip_prefix("--max-pages=") {
                    max_pages = match max_pages_str.parse::<i32>() {
                        Ok(max) => max,
                        Err(e) => {
                            println!("ERROR: Unable to parse '{max_pages_str}' as i32: {e}");
                            return None;
                        }
                    }
                } else {
                    println!("WARNING: Unknown parameter '{arg}'");
                }
            }
        }
    }

    if let Some(url) = url_res {
        return Some(
            CmdParams {
                url,
                unblur_nsfw,
                allow_nsfw,
                include_unavailable,
                limit,
                max_pages,
                vrc_only
            }
        );
    } else {
        println!("ERROR: No URL provided!");
        return None;
    }
}
