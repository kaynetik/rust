// check-fail
// Tests that a doc comment will not preclude a field from being considered a diagnostic argument

// The proc_macro2 crate handles spans differently when on beta/stable release rather than nightly,
// changing the output of this test. Since Subdiagnostic is strictly internal to the compiler
// the test is just ignored on stable and beta:
// ignore-stage1
// ignore-beta
// ignore-stable

#![feature(rustc_private)]
#![crate_type = "lib"]

extern crate rustc_errors;
extern crate rustc_fluent_macro;
extern crate rustc_macros;
extern crate rustc_session;
extern crate rustc_span;

use rustc_errors::{Applicability, DiagnosticMessage, SubdiagnosticMessage};
use rustc_fluent_macro::fluent_messages;
use rustc_macros::{Diagnostic, Subdiagnostic};
use rustc_span::Span;

fluent_messages! { "./example.ftl" }

struct NotIntoDiagnosticArg;

#[derive(Diagnostic)]
//~^ ERROR the trait bound `NotIntoDiagnosticArg: IntoDiagnosticArg` is not satisfied
#[diag(no_crate_example)]
struct Test {
    #[primary_span]
    span: Span,
    /// A doc comment
    arg: NotIntoDiagnosticArg,
}

#[derive(Subdiagnostic)]
//~^ ERROR the trait bound `NotIntoDiagnosticArg: IntoDiagnosticArg` is not satisfied
#[label(no_crate_example)]
struct SubTest {
    #[primary_span]
    span: Span,
    /// A doc comment
    arg: NotIntoDiagnosticArg,
}
