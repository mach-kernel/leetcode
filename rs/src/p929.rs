use std::collections::HashSet;

pub fn delivery_addr(email: &String) -> String {
    let parts: Vec<&str> = email.split("@").collect();

    if let Some(name) = parts.first() {
        let flat = name.replace(".", "");
        let tag = flat.split("+").next().map(|s| s.to_string()).unwrap_or(flat);
        format!("{}@{}", tag, parts.last().unwrap())
    } else {
        email.to_string()
    }
}

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    HashSet::<String>::from_iter(emails.iter().map(delivery_addr)).len() as i32
}

#[cfg(test)]
mod test {
    use crate::p929::num_unique_emails;

    #[test]
    fn test_num_unique_emails() {
        assert_eq!(num_unique_emails(vec!["a@leetcode.com".to_string(),"b@leetcode.com".to_string(),"c@leetcode.com".to_string()]), 3);
    }
}