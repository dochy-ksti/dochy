use crate::make_manual::manual_builder::ManualBuilderItem;
use crate::make_manual::make_page::l;
use crate::make_manual::get_content::get_md_filename;
use dochy::error::DpResult;

pub(crate) fn make_index_page(vec : &[ManualBuilderItem]) -> DpResult<String>{
    let mut r = String::new();
    {
        let r = &mut r;
        l(r, "## Dochy User's Manual");
        l(r, "");

        for item in vec {
            let src = item.src();
            if src.len() != 0 {
                l(r, &format!("### [{}]({})", item.title(), get_md_filename(item.src())?));
            } else{
                l(r, &format!("### {}", item.title()));
            }
            l(r, "");
        }
    }
    Ok(r)
}