// use json_a::*;
use json_proc::*;

fn ting(value: f32) -> f32 {
    value
}

#[warn_json(duplicate_keys)]
fn main() {
    let value = String::from("ga");
    let other_value = 1f32/32f32;
    
    println!("{}", json!({
        "hello": (2 + 4) as f32 + other_value + (b'e' as f32) + ting(100.0),
        "e": String::from("hello"),
        "test": None::<()>,
        "not": "trueStr",
        "embedded_array": [
            1,2,
            5
            ,10,
            {
                "heck": true
            }

        ]
        ,
        e2: false,
        e2: format!("hello: {} {hello}", "world!", hello = value)
    }));

    // println!("{}", core::any::type_name_of_val(&None::<String>));
}