//! ```cargo
//! [dependencies]
//! serde_json = "1.0.140"
//! minijinja = { version = "2.10.2", features = ["loader"] }
//! ```

use minijinja::{Environment, context};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = Environment::new();
    env.add_template("hello.txt", "Hello {{ name }}!").unwrap();
    let template = env.get_template("hello.txt").unwrap();
    println!("{}", template.render(context! { name => "World" }).unwrap());

    let a = r#"
        {% raw %}
            <ul>
            {% for item in seq %}
                <li>{{ item }}</li>
            {% endfor %}
            </ul>
        {% endraw %}
    "#;
    env.add_template("a", a).unwrap();
    println!("{}", env.get_template("a").unwrap().render(context!{}).unwrap());

    let b = r#"{% for i in range(rank - 1) -%} {{ "{}," -}} {% endfor %}"#;
    env.add_template("b", b).unwrap();
    println!("{}", env.get_template("b").unwrap().render(context!{ rank => 6 }).unwrap());

    let c = r#"{"action":"join","data":{"children":[{%for i in range(rank - 1)-%}{},{% endfor %}{{payload}}],"id":{{id}},"type":"case"{%if kind%},"kind":"{{kind}}"{%endif%}},"event":"chat"}"#;
    env.add_template("c", c).unwrap();
    let c = context!{rank => 8, payload => r#"{"a":123}"#};
    println!("{}", env.get_template("c").unwrap().render(c).unwrap());

    Ok(())
}
