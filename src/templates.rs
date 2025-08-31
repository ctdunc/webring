use crate::RingMember;
use std::collections::HashMap;
pub fn member_entry(member: &RingMember) -> String {
    return format!(
        "<b><a href=\"{}\">{}</a></b> by {}
            ",
        member.url, member.title, member.author
    );
}

pub fn homepage(members: &HashMap<String, RingMember>) -> String {
    let header = String::from(
        "<!DOCTYPE html>
<html>
<head>
    <title>Commutative Webring</title>
</head>
",
    );

    let mut body = String::from("<body><h1> Friends of Gauss </h1>");
    for (_, member) in members {
        body.push_str(member_entry(&member).as_str());
    }
    body.push_str("</body></html>");
    return format!("{}{}", header, body);
}
