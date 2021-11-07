use crate::make_manual::get_content::{get_content, get_file_stem, get_md_filename};
use dochy::error::DpResult;

pub(crate) fn make_page(prev_src: Option<&str>, next_src: Option<&str>, title : &str, src : &str)
                        -> DpResult<String>{

    let mut r = String::new();
    {
        let r = &mut r;
        if let Some(prev) = prev_src {
            l(r, &format!("[prev]({}.md)", get_md_filename(prev)?));
        }
        l(r, "[index](index.md)");
        if let Some(next) = next_src {
            l(r, &format!("[next]({}.md)", get_md_filename(next)?));
        }
        r.push('\n');
        l(r, &format!("### {}", title));
        r.push('\n');
        let content = get_content(src)?;
        l(r, &content);
        r.push('\n');
        r.push('\n');
        if let Some(prev) = prev_src {
            l(r, &format!("[prev]({}.md)", get_md_filename(prev)?));
        }
        l(r, "[index](index.md)");
        if let Some(next) = next_src {
            l(r, &format!("[next]({}.md)", get_md_filename(next)?));
        }
    }
    Ok(r)
}

pub(crate) fn l(r : &mut String, s : &str){
    r.push_str(s);
    r.push('\n');
}