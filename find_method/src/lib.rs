#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::{diagnostics::span_lint};
// use if_chain::if_chain;
// use rustc_ast::{
//     tokenstream:: NodeId,
// };
use rustc_ast::NodeId;
use rustc_lint::{EarlyContext, EarlyLintPass};

use rustc_ast::visit::FnKind;
use rustc_span::Span;

// use rustc_span::sym;

dylint_linting::declare_early_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub FIND_METHOD,
    Warn,
    "description goes here"
}

// impl<'tcx> LateLintPass<'tcx> for FindMethod {
//     // A list of things you might check can be found here:
//     // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html
// }

// #[derive(Default)]
// pub struct FindMethod {
// }

impl EarlyLintPass for FindMethod {

    fn check_fn(&mut self, cx: &EarlyContext<'_>, fn_kind: FnKind<'_>, span: Span, _: NodeId) {
        if is_foo_fn(fn_kind) {
            span_lint(
                cx,
                FIND_METHOD,
                span,
                "function named `add` fue encontrada!",
            );
        }
    }
}

fn is_foo_fn(fn_kind: FnKind<'_>) -> bool {
    match fn_kind {
        FnKind::Fn(_, ident, ..) => {
            // check if `fn` name is `foo`
            ident.name.as_str() == "add"
        }
        // ignore closures
        FnKind::Closure(..) => false
        // FnKind::Closure => false
    }
}

// fn is_blacklisted_variable(var: &str) -> bool {
//     var == "CARGO" || var == "CARGO_MANIFEST_DIR" || var.starts_with("CARGO_BIN_EXE_")
// }

#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
