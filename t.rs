#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn ting(value: f32) -> f32 {
    value
}
#[allow(dead_code)]
fn main() {
    tests::test()
}
mod tests {
    use std::time::Instant;
    use super::*;
    use json_proc::*;
    struct Test<T> {
        yes: String,
        test: T,
    }
    enum Test2 {
        Hello { hello: String },
        Two(String, u8),
    }
    impl ToJson for Test2 {
        fn to_json_string(&self) -> String {
            match self {
                Self::Hello { hello } => {
                    let mut pairs: Vec<(String, String)> = Vec::new();
                    {
                        let key = "hello";
                        let value = json_proc::ToJson::to_json_string(hello);
                        pairs.push((key.to_string(), value));
                    }
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{{{0}}}",
                                pairs
                                    .into_iter()
                                    .map(|(key, val)| ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("\"{0}\":{1}", key, val),
                                        );
                                        res
                                    }))
                                    .collect::<Vec<String>>()
                                    .join(","),
                            ),
                        );
                        res
                    })
                }
                Self::Two(arg0, arg1) => {
                    let values: Vec<String> = <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            json_proc::ToJson::to_json_string(arg0),
                            json_proc::ToJson::to_json_string(arg1),
                        ]),
                    );
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "[{0}]",
                                values
                                    .into_iter()
                                    .map(|val| ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(format_args!("{0}", val));
                                        res
                                    }))
                                    .collect::<Vec<String>>()
                                    .join(","),
                            ),
                        );
                        res
                    })
                }
            }
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test"]
    #[doc(hidden)]
    pub const test: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests\\main.rs",
            start_line: 37usize,
            start_col: 12usize,
            end_line: 37usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test())),
    };
    pub fn test() {
        {
            ::std::io::_print(format_args!("{0}\n", size_of::<bool>()));
        };
        let value = String::from("ga");
        let other_value = 1f32 / 32f32;
        let strc = Test {
            yes: String::from("hello"),
            test: -5,
        };
        {
            ::std::io::_print(
                format_args!(
                    "{0}\n",
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{{{0}}}",
                                {
                                    let vec: Vec<String> = <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "\"{0}\":{1}",
                                                            "hello",
                                                            json_proc::ToJson::to_json_string(
                                                                &((2 + 4) as f32 + other_value + (b'e' as f32)
                                                                    + ting(100.0)),
                                                            ),
                                                        ),
                                                    );
                                                    res
                                                })
                                                .to_string(),
                                            ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "\"{0}\":{1}",
                                                            "e",
                                                            json_proc::ToJson::to_json_string(&(String::from("hello"))),
                                                        ),
                                                    );
                                                    res
                                                })
                                                .to_string(),
                                            ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "\"{0}\":{1}",
                                                            "test",
                                                            json_proc::ToJson::to_json_string(&(None::<()>)),
                                                        ),
                                                    );
                                                    res
                                                })
                                                .to_string(),
                                            ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "\"{0}\":{1}",
                                                            "not",
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!("\"{0}\"", "trueStr"),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                    );
                                                    res
                                                })
                                                .to_string(),
                                            ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "\"{0}\":{1}",
                                                            "embedded_array",
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "[{0}]",
                                                                        <[_]>::into_vec(
                                                                                #[rustc_box]
                                                                                ::alloc::boxed::Box::new([
                                                                                    json_proc::ToJson::to_json_string(&(1)).to_string(),
                                                                                    json_proc::ToJson::to_json_string(&(2)).to_string(),
                                                                                    json_proc::ToJson::to_json_string(&(5)).to_string(),
                                                                                    json_proc::ToJson::to_json_string(&(10)).to_string(),
                                                                                    ::alloc::__export::must_use({
                                                                                            let res = ::alloc::fmt::format(
                                                                                                format_args!(
                                                                                                    "{{{0}}}",
                                                                                                    {
                                                                                                        let vec: Vec<String> = <[_]>::into_vec(
                                                                                                            #[rustc_box]
                                                                                                            ::alloc::boxed::Box::new([
                                                                                                                ::alloc::__export::must_use({
                                                                                                                        let res = ::alloc::fmt::format(
                                                                                                                            format_args!("\"{0}\":{1}", "heck", true),
                                                                                                                        );
                                                                                                                        res
                                                                                                                    })
                                                                                                                    .to_string(),
                                                                                                            ]),
                                                                                                        );
                                                                                                        vec
                                                                                                    }
                                                                                                        .join(","),
                                                                                                ),
                                                                                            );
                                                                                            res
                                                                                        })
                                                                                        .to_string(),
                                                                                ]),
                                                                            )
                                                                            .join(","),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                    );
                                                    res
                                                })
                                                .to_string(),
                                            ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("\"{0}\":{1}", "e2", false),
                                                    );
                                                    res
                                                })
                                                .to_string(),
                                            ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "\"{0}\":{1}",
                                                            "es2",
                                                            json_proc::ToJson::to_json_string(
                                                                &(::alloc::__export::must_use({
                                                                    let res = ::alloc::fmt::format(
                                                                        format_args!("hello: {0} {1}", "world!", value),
                                                                    );
                                                                    res
                                                                })),
                                                            ),
                                                        ),
                                                    );
                                                    res
                                                })
                                                .to_string(),
                                            ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "\"{0}\":{1}",
                                                            "test22",
                                                            json_proc::ToJson::to_json_string(&(strc)),
                                                        ),
                                                    );
                                                    res
                                                })
                                                .to_string(),
                                            ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "\"{0}\":{1}",
                                                            "test2Enum",
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "[{0}]",
                                                                        <[_]>::into_vec(
                                                                                #[rustc_box]
                                                                                ::alloc::boxed::Box::new([
                                                                                    json_proc::ToJson::to_json_string(
                                                                                            &(Test2::Hello {
                                                                                                hello: String::from("this is Hello"),
                                                                                            }),
                                                                                        )
                                                                                        .to_string(),
                                                                                    json_proc::ToJson::to_json_string(
                                                                                            &(Test2::Two(String::from("this is 2"), 2)),
                                                                                        )
                                                                                        .to_string(),
                                                                                ]),
                                                                            )
                                                                            .join(","),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                    );
                                                    res
                                                })
                                                .to_string(),
                                        ]),
                                    );
                                    vec
                                }
                                    .join(","),
                            ),
                        );
                        res
                    }),
                ),
            );
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::bench"]
    #[doc(hidden)]
    pub const bench: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::bench"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests\\main.rs",
            start_line: 73usize,
            start_col: 8usize,
            end_line: 73usize,
            end_col: 13usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(bench())),
    };
    fn bench() {
        let hello = String::from("bad");
        let start = Instant::now();
        serde_json::to_string(
                &::serde_json::Value::Object({
                    let mut object = ::serde_json::Map::new();
                    let _ = object
                        .insert(
                            ("hello").into(),
                            ::serde_json::to_value(&String::from("thisIsAString"))
                                .unwrap(),
                        );
                    let _ = object
                        .insert(
                            ("struct").into(),
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("yes").into(),
                                        ::serde_json::to_value(&String::new()).unwrap(),
                                    );
                                object
                            }),
                        );
                    let _ = object
                        .insert(
                            (hello.clone()).into(),
                            ::serde_json::to_value(&hello).unwrap(),
                        );
                    object
                }),
            )
            .unwrap();
        {
            ::std::io::_print(format_args!("{0:?}\n", start.elapsed()));
        };
        let start = Instant::now();
        let _ = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!(
                    "{{{0}}}",
                    {
                        let vec: Vec<String> = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "hello",
                                                json_proc::ToJson::to_json_string(
                                                    &(String::from("thisIsAString")),
                                                ),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "struct",
                                                json_proc::ToJson::to_json_string(
                                                    &(Test {
                                                        yes: String::new(),
                                                        test: usize::MAX,
                                                    }),
                                                ),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "bad",
                                                json_proc::ToJson::to_json_string(&(hello)),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                            ]),
                        );
                        vec
                    }
                        .join(","),
                ),
            );
            res
        });
        {
            ::std::io::_print(format_args!("{0:?}\n", start.elapsed()));
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_basic_key_value_pairs"]
    #[doc(hidden)]
    pub const test_basic_key_value_pairs: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_basic_key_value_pairs"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests\\main.rs",
            start_line: 97usize,
            start_col: 8usize,
            end_line: 97usize,
            end_col: 34usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_basic_key_value_pairs()),
        ),
    };
    fn test_basic_key_value_pairs() {
        let json_str = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!(
                    "{{{0}}}",
                    {
                        let vec: Vec<String> = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "name",
                                                ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("\"{0}\"", "John Doe"),
                                                    );
                                                    res
                                                }),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "age",
                                                json_proc::ToJson::to_json_string(&(30)),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("\"{0}\":{1}", "active", true),
                                        );
                                        res
                                    })
                                    .to_string(),
                            ]),
                        );
                        vec
                    }
                        .join(","),
                ),
            );
            res
        });
        match (
            &serde_json::to_string(&::serde_json::to_value(&json_str).unwrap()).unwrap(),
            &serde_json::to_string(
                    &::serde_json::to_value(
                            &r#"{"name":"John Doe","age":30,"active":true}"#,
                        )
                        .unwrap(),
                )
                .unwrap(),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_string_from"]
    #[doc(hidden)]
    pub const test_string_from: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_string_from"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests\\main.rs",
            start_line: 107usize,
            start_col: 8usize,
            end_line: 107usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_string_from()),
        ),
    };
    fn test_string_from() {
        let json_str = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!(
                    "{{{0}}}",
                    {
                        let vec: Vec<String> = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "greeting",
                                                json_proc::ToJson::to_json_string(
                                                    &(String::from("Hello, world!")),
                                                ),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                            ]),
                        );
                        vec
                    }
                        .join(","),
                ),
            );
            res
        });
        match (
            &serde_json::to_string(&::serde_json::to_value(&json_str).unwrap()).unwrap(),
            &serde_json::to_string(
                    &::serde_json::to_value(&r#"{"greeting":"Hello, world!"}"#).unwrap(),
                )
                .unwrap(),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_nested_json"]
    #[doc(hidden)]
    pub const test_nested_json: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_nested_json"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests\\main.rs",
            start_line: 115usize,
            start_col: 8usize,
            end_line: 115usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_nested_json()),
        ),
    };
    fn test_nested_json() {
        let json_str = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!(
                    "{{{0}}}",
                    {
                        let vec: Vec<String> = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "user",
                                                ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "{{{0}}}",
                                                            {
                                                                let vec: Vec<String> = <[_]>::into_vec(
                                                                    #[rustc_box]
                                                                    ::alloc::boxed::Box::new([
                                                                        ::alloc::__export::must_use({
                                                                                let res = ::alloc::fmt::format(
                                                                                    format_args!(
                                                                                        "\"{0}\":{1}",
                                                                                        "name",
                                                                                        ::alloc::__export::must_use({
                                                                                            let res = ::alloc::fmt::format(
                                                                                                format_args!("\"{0}\"", "Jane Doe"),
                                                                                            );
                                                                                            res
                                                                                        }),
                                                                                    ),
                                                                                );
                                                                                res
                                                                            })
                                                                            .to_string(),
                                                                        ::alloc::__export::must_use({
                                                                                let res = ::alloc::fmt::format(
                                                                                    format_args!(
                                                                                        "\"{0}\":{1}",
                                                                                        "age",
                                                                                        json_proc::ToJson::to_json_string(&(25)),
                                                                                    ),
                                                                                );
                                                                                res
                                                                            })
                                                                            .to_string(),
                                                                    ]),
                                                                );
                                                                vec
                                                            }
                                                                .join(","),
                                                        ),
                                                    );
                                                    res
                                                }),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("\"{0}\":{1}", "is_admin", false),
                                        );
                                        res
                                    })
                                    .to_string(),
                            ]),
                        );
                        vec
                    }
                        .join(","),
                ),
            );
            res
        });
        match (
            &serde_json::to_string(&::serde_json::to_value(&json_str).unwrap()).unwrap(),
            &serde_json::to_string(
                    &::serde_json::to_value(
                            &r#"{"user":{"name":"Jane Doe","age":25},"is_admin":false}"#,
                        )
                        .unwrap(),
                )
                .unwrap(),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    #[allow(dead_code)]
    fn test_json_with_escape_characters() {
        let json_str = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!(
                    "{{{0}}}",
                    {
                        let vec: Vec<String> = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "message",
                                                ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("\"{0}\"", "This is a \"quoted\" word."),
                                                    );
                                                    res
                                                }),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "newline",
                                                ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("\"{0}\"", "First line.\nSecond line."),
                                                    );
                                                    res
                                                }),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                            ]),
                        );
                        vec
                    }
                        .join(","),
                ),
            );
            res
        });
        match (
            &serde_json::to_string(&::serde_json::to_value(&json_str).unwrap()).unwrap(),
            &serde_json::to_string(
                    &::serde_json::to_value(
                            &"{\"message\":\"This is a \\\"quoted\\\" word.\",\"newline\":\"First line.\\nSecond line.\"}",
                        )
                        .unwrap(),
                )
                .unwrap(),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_empty_json"]
    #[doc(hidden)]
    pub const test_empty_json: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_empty_json"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests\\main.rs",
            start_line: 136usize,
            start_col: 8usize,
            end_line: 136usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_empty_json()),
        ),
    };
    fn test_empty_json() {
        let json_str = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!(
                    "{{{0}}}",
                    {
                        let vec: Vec<String> = ::alloc::vec::Vec::new();
                        vec
                    }
                        .join(","),
                ),
            );
            res
        });
        match (
            &serde_json::to_string(&::serde_json::to_value(&json_str).unwrap()).unwrap(),
            &serde_json::to_string(&::serde_json::to_value(&r#"{}"#).unwrap()).unwrap(),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_complex_expressions"]
    #[doc(hidden)]
    pub const test_complex_expressions: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_complex_expressions"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests\\main.rs",
            start_line: 142usize,
            start_col: 8usize,
            end_line: 142usize,
            end_col: 32usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_complex_expressions()),
        ),
    };
    fn test_complex_expressions() {
        let number = 42;
        let json_str = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!(
                    "{{{0}}}",
                    {
                        let vec: Vec<String> = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "answer",
                                                json_proc::ToJson::to_json_string(&(number)),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "\"{0}\":{1}",
                                                "text",
                                                json_proc::ToJson::to_json_string(
                                                    &(String::from("The answer is")),
                                                ),
                                            ),
                                        );
                                        res
                                    })
                                    .to_string(),
                            ]),
                        );
                        vec
                    }
                        .join(","),
                ),
            );
            res
        });
        match (
            &serde_json::to_string(&::serde_json::to_value(&json_str).unwrap()).unwrap(),
            &serde_json::to_string(
                    &::serde_json::to_value(&r#"{"answer":42,"text":"The answer is"}"#)
                        .unwrap(),
                )
                .unwrap(),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &bench,
            &test,
            &test_basic_key_value_pairs,
            &test_complex_expressions,
            &test_empty_json,
            &test_nested_json,
            &test_string_from,
        ],
    )
}
