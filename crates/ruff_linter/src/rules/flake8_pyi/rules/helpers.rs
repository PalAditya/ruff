use ruff_python_ast as ast;
use ruff_python_parser::typing::parse_type_annotation;
use ruff_source_file::Locator;

/// Apply a test to an annotation expression,
/// abstracting over the fact that the annotation expression might be "stringized".
///
/// A stringized annotation is one enclosed in string quotes:
/// `foo: "typing.Any"` means the same thing to a type checker as `foo: typing.Any`.
pub(super) fn match_maybe_stringized_annotation(
    expr: &ast::Expr,
    locator: &Locator,
    match_fn: impl FnOnce(&ast::Expr) -> bool,
) -> bool {
    if let ast::Expr::StringLiteral(string_annotation) = expr {
        let Ok((parsed_annotation, _)) =
            parse_type_annotation(string_annotation, locator.contents())
        else {
            return false;
        };
        if !parsed_annotation.errors().is_empty() {
            return false;
        }
        let ast::ModExpression {
            body: annotation, ..
        } = parsed_annotation.into_syntax();
        match_fn(&annotation)
    } else {
        match_fn(expr)
    }
}
