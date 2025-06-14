use rustdoc_types::ItemEnum;

use crate::LintId;

/// Point out enums which are sorted but cannot be kept sorted for future additions.
pub(crate) fn check(context: &crate::Context, d: &mut crate::Diagnostics<'_>) {
    'next_type: for item in context.public_api.items() {
        let crate_item = context.crate_.index.get(&item.id()).unwrap();

        let ItemEnum::Enum(e) = &crate_item.inner else {
            continue;
        };

        let mut variants = vec![];

        for v in e.variants.iter() {
            let variant_item = context.crate_.index.get(v).unwrap();
            let ItemEnum::Variant(v) = &variant_item.inner else {
                continue;
            };

            if v.discriminant.is_some() {
                continue 'next_type;
            }

            if variant_item.attrs.iter().any(|s| s == NON_EXHAUSTIVE) {
                continue 'next_type;
            }

            variants.push(variant_item.name.as_ref().cloned().unwrap());
        }

        if !variants.is_sorted() {
            continue 'next_type;
        }

        d.diagnostic(
            LintId::SortedEnum,
            &format!(
                "enum `{item}` has its variants sorted, but this cannot be respected for additions"
            ),
            item,
            crate_item.span.as_ref(),
        );
    }
}

const NON_EXHAUSTIVE: &str = "#[non_exhaustive]";
