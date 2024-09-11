pub const PYTHON_APP: [(&str, &str); 2] = [
("utils.foo", "def bar():\n    return \"baz\""),
("other_module.run_foo", "from utils.foo import bar\n\ndef run_foo():\n    return bar()"),
];
