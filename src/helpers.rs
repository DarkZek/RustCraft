use std::ops::Add;

fn format_u32(mut count: u32) -> String {
    let mut msg = String::new();

    while count != 0 {
        if count / 1000 == 0 {
            msg = (count % 1000).to_string().add(msg.as_str());
        } else {
            msg = format!(",{}", count % 1000).add(msg.as_str());
        }

        count = count / 1000;
    }

    msg
}
