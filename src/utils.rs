use comrak::nodes::AstNode;

use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

pub fn highlight_text(text: String, lang: String) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();

    let syntax = syntax_set
        .find_syntax_by_extension(&lang)
        .unwrap_or(syntax_set.find_syntax_plain_text());

    let mut rs_html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);

    for line in LinesWithEndings::from(&text) {
        let res = rs_html_generator.parse_html_for_line_which_includes_newline(&line);
        if res.is_err() {
            return String::new();
        }
    }

    rs_html_generator.finalize()
}

pub fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}
