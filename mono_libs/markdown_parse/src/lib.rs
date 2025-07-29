pub fn markdown_to_html(mkdn: String) -> String {
    let mut to_ret:String = String::new();
    let mut begin:usize = 0;
    let mut end:usize = 0;
    let mut mid:String = String::new();
    let mut is_list:bool = false;
    let mut list_tag:String = String::from("ul");
    let mut testing:String = String::from("kms");
   for (i, &item) in mkdn.as_bytes().iter().enumerate() {
       if item == b'\n' {
            end = i as usize;
            mid = mid_tag((mkdn[begin..end].to_string()));
            testing = mid[0..7].to_string();
            println!("{}",testing);
            if mid[0..4] == String::from("<li>") {
                if mid[4..6] == String::from("- ") {
                    to_ret.push_str(&format!("<{list_tag}>"));
                    mid.replace_range(4..6, "");
                } else {
                    list_tag = String::from("ol");
                    to_ret.push_str(&format!("<{list_tag}>"));
                    mid.replace_range(4..6, "");
                }
                if is_list == false {
                    is_list = true;
                    println!("changed is_list");
                }
            } else if is_list == true {
                to_ret.push_str(&format!("</{list_tag}>"));
            }
            to_ret.push_str(&mid);
            begin = i + 1 as usize;
       }
   }
   let end = mkdn.len() as usize;
   to_ret.push_str(&mid_tag((mkdn[begin..end]).to_string()));
    return to_ret; 
}

fn mid_tag(mkdn: String) -> String{
    let mut tag:String = String::from("p");
    let mut ret:String = String::new();
    let mut head:u8 = 0;
    let mut asts:u8 = 0;
    let mut close_ast:bool = false;
    let mut close_asts:u8 = 0;
    let mut first_item:bool = true;
    for (i, &item) in mkdn.as_bytes().iter().enumerate() {
        let VAL:bool = item.is_ascii_alphabetic();
        match item {
            b'#' => {
                if first_item == true {
                    head = head + 1;
                } else {
                    ret.push_str("#");
                }
                },
            b'*' => {
                if first_item == true {
                    first_item = false;
                }
                if close_ast == true {
                    if close_asts + 1 == asts {
                        match close_asts {
                            0 => ret.push_str("</em>"),
                            1 => ret.push_str("</strong>"),
                            2 => ret.push_str("</em></strong>"),   
                            _ => (),
                        }
                        asts =0;
                        close_asts = 0;
                        close_ast = false;
                } else {
                    close_asts = close_asts + 1;
                    }
                } else {
                    asts = asts + 1;
                }
            },
            b'-' => {
                tag = format!("li");
                if first_item == true {
                    first_item = false;
                }
                ret.push_str("-");
            },
            VAL => {
                if first_item == true {
                    first_item = false;
                }
                if asts > 0 && close_ast == false {
                    close_ast = true;
                    match asts {
                        1 => ret.push_str("<em>"),
                        2 => ret.push_str("<strong>"),
                        3 => ret.push_str("<em><strong>"),
                        _ => (),
                    }
                }
                let s = (item as char).to_string();
               ret.push_str(&s);
            },
        }
    }
    if head > 0 {
        tag = format!("h{head}");
    }
    let return_str:String = format!("<{tag}>{ret}</{tag}>");
    tag = format!("p");
    return return_str;

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_p() {
        let result:String = markdown_to_html(String::from("this is a paragraph"));
        println!("{result}");
        assert_eq!(result, String::from("<p>this is a paragraph</p>"));
    }
    #[test]
    fn norm_h() {
        let result:String = markdown_to_html(String::from("## this is a heading"));
        assert_eq!(result, String::from("<h2> this is a heading</h2>"));
    }

    #[test]
    fn ital_and_bold() {
        let result:String = markdown_to_html(String::from("*this* is a **strong** ***paragraph***"));
        assert_eq!(result, String::from("<p><em>this</em> is a <strong>strong</strong> <em><strong>paragraph</em></strong></p>"));
    }

    #[test]
    fn multi_line() {
        let result:String = markdown_to_html(String::from("# This is the heading \n This is a paragraph. *This part is italicized,* **and this part is bold.**"));
        assert_eq!(result, String::from("<h1> This is the heading </h1><p> This is a paragraph. <em>This part is italicized,</em> <strong>and this part is bold.</strong></p>"));
    }

    #[test]
    fn mid_hash() {
        let result:String = markdown_to_html(String::from("This is a paragraph that contains # as a char"));
        assert_eq!(result, String::from("<p>This is a paragraph that contains # as a char</p>"));
    }

    #[test]
    fn listing() {
        let result:String = markdown_to_html(String::from("- this is a list item.\n now we add."));
        assert_eq!(result, String::from("<ul><li>this is a list item.</li></ul><p> now we add.</p>"));
    }
}
