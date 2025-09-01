use crate::RingMember;

use std::collections::HashMap;
pub fn member_entry(member: &RingMember) -> String {
    return format!(
        "<p><b><a href=\"{}\">{}</a></b> by {}</p>",
        member.url, member.title, member.author
    );
}

pub fn homepage(members: &HashMap<String, RingMember>, title: &String) -> String {
    let mut list = String::from("<ul>");
    for (_, member) in members {
        list.push_str(member_entry(&member).as_str());
    }
    list.push_str("</body></html>");
    return format!(include_str!("../template.html"), title = title, list = list);
}
