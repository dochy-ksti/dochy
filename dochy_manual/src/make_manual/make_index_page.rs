use crate::make_manual::manual_builder::ManualBuilderItem;
use crate::make_manual::make_page::l;
use crate::make_manual::get_content::get_md_filename;
use dochy::error::DpResult;
use dochy::core::structs::NullOr;

pub(crate) fn make_index_page(vec : &[ManualBuilderItem]) -> DpResult<String>{
    let mut r = String::new();
    {
        let r = &mut r;
        l(r, "## Dochy User's Manual");
        l(r, "");

        for item in vec {
            let src = item.src();
            if let NullOr::Val(src) = src {
                match get_indent(item.title()){
                    0 => l(r, &format!("### [{}]({})", item.title(), get_md_filename(src)?)),
                    1 => l(r, &format!("#### 　[{}]({})", item.title(), get_md_filename(src)?)),
                    2 => l(r, &format!("##### 　　[{}]({})", item.title(), get_md_filename(src)?)),
                    _ => unreachable!(),
                }

            } else{
                l(r, &format!("### {}", item.title()));
            }
            l(r, "");
        }
    }
    Ok(r)
}

fn get_indent(title : &str) -> usize{
    title.chars()
        .take_while(|c| *c != '.')
        .filter(|c| *c == '-')
        .count()
}