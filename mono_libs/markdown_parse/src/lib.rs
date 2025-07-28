pub fn markdown_to_html(mkdn: String) -> String {
    let mut to_ret:String = String::new();
    let mut begin:usize = 0;
    let mut end:usize = 0;

   for (i, &item) in mkdn.as_bytes().iter().enumerate() {
       if item == b'\n' {
            let end = i as usize;
            to_ret.push_str(&mid_tag((mkdn[begin..end].to_string())));
            let begin = i + 1;
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
    for (i, &item) in mkdn.as_bytes().iter().enumerate() {
        let VAL:bool = item.is_ascii_alphabetic();
        match item {
            b'#' => head = head + 1,
            b'*' => {
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
            VAL => {
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
    fn bold_p() {
        let result:String = markdown_to_html(String::from("this is a **strong** paragraph"));
        assert_eq!(result, String::from("<p>this is a <strong>strong</strong> paragraph</p>"));
    }
}
