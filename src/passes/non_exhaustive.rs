use crate::LintId;
use rustdoc_types::{ItemEnum, StructKind};

/// Issue a diagnostic for each `pub enum` which is not `#[non_exhaustive]`
pub(crate) fn check_enums(context: &crate::Context, d: &mut crate::Diagnostics<'_>) {
    for item in context.public_api.items() {
        let crate_item = context.crate_.index.get(&item.id()).unwrap();

        if let ItemEnum::Enum(_) = &crate_item.inner {
            if !crate_item.attrs.iter().any(|s| s == NON_EXHAUSTIVE) {
                d.diagnostic(
                    LintId::ExhaustiveStruct,
                    &format!("enum `{item}` should be marked {NON_EXHAUSTIVE}"),
                    item,
                    crate_item.span.as_ref(),
                );
            }
        }
    }
}

/// Issue a diagnostic for each `pub struct` which is not `#[non_exhaustive]`
pub(crate) fn check_structs(context: &crate::Context, d: &mut crate::Diagnostics<'_>) {
    for item in context.public_api.items() {
        let crate_item = context
            .crate_
            .index
            .get(&item.id())
            .expect("public_api contains item not in crate");

        if let ItemEnum::Struct(s) = &crate_item.inner {
            // structs with at least one non-public item do not benefit from
            // being marked #[non_exhaustive]
            match &s.kind {
                StructKind::Plain {
                    has_stripped_fields: true,
                    ..
                } => {
                    continue;
                }
                StructKind::Tuple(ids) => {
                    if ids.contains(&None) {
                        continue;
                    }
                }

                StructKind::Plain {
                    has_stripped_fields: false,
                    ..
                } => {}
                StructKind::Unit => {}
            }

            if !crate_item.attrs.iter().any(|s| s == NON_EXHAUSTIVE) {
                d.diagnostic(
                    LintId::ExhaustiveStruct,
                    &format!("struct `{item}` should be marked {NON_EXHAUSTIVE}"),
                    item,
                    crate_item.span.as_ref(),
                );
            }
        }
    }
}

const NON_EXHAUSTIVE: &str = "#[non_exhaustive]";
