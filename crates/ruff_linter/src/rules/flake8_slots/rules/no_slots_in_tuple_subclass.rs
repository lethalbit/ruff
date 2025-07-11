use ruff_python_ast::{Arguments, Stmt, StmtClassDef};

use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::helpers::map_subscript;
use ruff_python_ast::identifier::Identifier;

use crate::Violation;
use crate::checkers::ast::Checker;
use crate::rules::flake8_slots::helpers::has_slots;

/// ## What it does
/// Checks for subclasses of `tuple` that lack a `__slots__` definition.
///
/// ## Why is this bad?
/// In Python, the `__slots__` attribute allows you to explicitly define the
/// attributes (instance variables) that a class can have. By default, Python
/// uses a dictionary to store an object's attributes, which incurs some memory
/// overhead. However, when `__slots__` is defined, Python uses a more compact
/// internal structure to store the object's attributes, resulting in memory
/// savings.
///
/// Subclasses of `tuple` inherit all the attributes and methods of the
/// built-in `tuple` class. Since tuples are typically immutable, they don't
/// require additional attributes beyond what the `tuple` class provides.
/// Defining `__slots__` for subclasses of `tuple` prevents the creation of a
/// dictionary for each instance, reducing memory consumption.
///
/// ## Example
/// ```python
/// class Foo(tuple):
///     pass
/// ```
///
/// Use instead:
/// ```python
/// class Foo(tuple):
///     __slots__ = ()
/// ```
///
/// ## References
/// - [Python documentation: `__slots__`](https://docs.python.org/3/reference/datamodel.html#slots)
#[derive(ViolationMetadata)]
pub(crate) struct NoSlotsInTupleSubclass;

impl Violation for NoSlotsInTupleSubclass {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Subclasses of `tuple` should define `__slots__`".to_string()
    }
}

/// SLOT001
pub(crate) fn no_slots_in_tuple_subclass(checker: &Checker, stmt: &Stmt, class: &StmtClassDef) {
    // https://github.com/astral-sh/ruff/issues/14535
    if checker.source_type.is_stub() {
        return;
    }
    let Some(Arguments { args: bases, .. }) = class.arguments.as_deref() else {
        return;
    };

    let semantic = checker.semantic();

    if bases.iter().any(|base| {
        let base = map_subscript(base);
        semantic.match_builtin_expr(base, "tuple") || semantic.match_typing_expr(base, "Tuple")
    }) {
        if !has_slots(&class.body) {
            checker.report_diagnostic(NoSlotsInTupleSubclass, stmt.identifier());
        }
    }
}
