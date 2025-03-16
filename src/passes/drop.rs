use public_api::PublicItem;
use rustdoc_types::{Id, Item, ItemEnum};

/// Issue a diagnostic for each aggregate type that is not `Drop`
pub(crate) fn check(context: &crate::Context, d: &mut crate::Diagnostics<'_>) {
    let drop_trait = context.find_trait(&["core", "ops", "drop", "Drop"]);
    let copy_trait = context.find_trait(&["core", "marker", "Copy"]);

    for item in context.public_api.items() {
        let crate_item = context.crate_.index.get(&item.id()).unwrap();

        match &crate_item.inner {
            ItemEnum::Enum(e) => {
                check_composite(d, item, crate_item, &e.impls, drop_trait, copy_trait);
            }
            ItemEnum::Struct(s) => {
                check_composite(d, item, crate_item, &s.impls, drop_trait, copy_trait);
            }
            _ => {}
        }
    }
}

fn check_composite(
    d: &mut crate::Diagnostics<'_>,
    item: &PublicItem,
    crate_item: &Item,
    impls: &[Id],
    drop_trait: Option<Id>,
    copy_trait: Option<Id>,
) {
    if let Some(copy_id) = &copy_trait {
        if impls.contains(copy_id) {
            // If T impls Copy, then it cannot impl Drop; and cannot have an impl-Drop
            // field added later
            return;
        }
    }

    if let Some(drop_id) = &drop_trait {
        if impls.contains(drop_id) {
            return;
        }
    }

    let kind = match &crate_item.inner {
        ItemEnum::Enum(_) => "enum",
        ItemEnum::Struct(_) => "struct",
        _ => unreachable!(),
    };
    d.diagnostic(
        crate::LintId::Drop,
        &format!("{kind} `{item}` should impl Drop"),
        item,
        crate_item.span.as_ref(),
    );
}
