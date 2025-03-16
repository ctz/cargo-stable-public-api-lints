use clap::{Parser, ValueEnum};
use rustdoc_types::{Id, ItemKind};

mod passes {
    pub(super) mod drop;
    pub(super) mod non_exhaustive;
}

/// This tool implements some lints intended to help crate authors
/// expose a stable public API.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The nightly toolchain version to use for producing rustdoc JSON.
    ///
    /// This needs to match the version supported by this tool's
    /// rustdoc_json's dependency.
    #[clap(long, default_value = "nightly")]
    toolchain: String,

    /// Where to find the crate to inspect.
    #[clap(default_value = "Cargo.toml")]
    manifest_path: String,

    /// Allow the named diagnostic.
    ///
    /// Use `--allow all` to disable all diagnostics by default.  Then
    /// individual diagnostics can be reenabled with `--warn ID`.
    #[clap(long, short)]
    allow: Vec<LintId>,

    /// Warn on the named diagnostic.
    ///
    /// This is the default for all diagnostics.
    #[clap(long, short)]
    warn: Vec<LintId>,

    /// The --no-default-features option, passed to `cargo doc`
    #[clap(long, default_value_t = false)]
    no_default_features: bool,

    /// The --all-features option, passed to `cargo doc`
    #[clap(long, default_value_t = false)]
    all_features: bool,

    /// The --features option, passed to `cargo doc`
    #[clap(long)]
    features: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // remove extra process name if invoked by cargo
    let mut args = std::env::args().collect::<Vec<String>>();
    if let Some(first) = args.get(1) {
        if first == "stable-public-api-lints" {
            args.drain(1..2);
        }
    }

    let cli = Cli::parse_from(args);

    // Set RUSTFLAGS--cfg=spal
    // SAFETY: currently we are the only thread
    unsafe {
        std::env::set_var("RUSTFLAGS", "--cfg=spal");
    }

    let json_path = rustdoc_json::Builder::default()
        .toolchain(cli.toolchain.clone())
        .manifest_path(cli.manifest_path.clone())
        .all_features(cli.all_features)
        .features(cli.features.clone())
        .no_default_features(cli.no_default_features)
        .build()?;

    // need to load this twice, as `public_api` doesn't expose its underlying
    // value of `rustdoc_types::Crate`
    let public_api = public_api::Builder::from_rustdoc_json(&json_path).build()?;
    let crate_: rustdoc_types::Crate = {
        let json_str = std::fs::read_to_string(json_path)?;
        let mut deserializer = serde_json::Deserializer::from_str(&json_str);
        deserializer.disable_recursion_limit();
        serde::de::Deserialize::deserialize(&mut deserializer)?
    };

    let ctx = Context { crate_, public_api };
    let mut diagnostics = Diagnostics {
        context: &ctx,
        cli,
        lints_output: vec![],
    };
    passes::non_exhaustive::check_enums(&ctx, &mut diagnostics);
    passes::non_exhaustive::check_structs(&ctx, &mut diagnostics);
    passes::drop::check(&ctx, &mut diagnostics);

    if diagnostics.lints_output.is_empty() {
        Ok(())
    } else {
        std::process::exit(1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
enum LintId {
    All,
    ExhaustiveStruct,
    ExhaustiveEnum,
    Drop,
}

impl LintId {
    fn qualified_name(&self) -> &str {
        match self {
            Self::All => "spal::all",
            Self::ExhaustiveStruct => "spal::exhaustive_struct",
            Self::ExhaustiveEnum => "spal::exhaustive_enum",
            Self::Drop => "spal::drop",
        }
    }

    fn long_form_explanation(&self) -> &str {
        match self {
            Self::All => unreachable!(),
            Self::ExhaustiveStruct => {
                "Marking a struct `#[non_exhaustive]` means future changes can add new fields."
            }
            Self::ExhaustiveEnum => {
                "Marking an enum `#[non_exhaustive]` means future changes can add new variants."
            }
            Self::Drop => {
                "It is a breaking change for a type to become `Drop` later, and this also happens\n\
                \"magically\" when any type reachable through any field or variant becomes `Drop`.\n\
                This includes the types of private fields!\n\n\
                Therefore it is prudent to `impl Drop` up-front to allow the maximum amount of\n\
                future non-breaking changes, both in your code and the code of your dependencies."
            }
        }
    }
}

struct Diagnostics<'a> {
    context: &'a Context,
    cli: Cli,
    lints_output: Vec<LintId>,
}

impl Diagnostics<'_> {
    fn diagnostic(
        &mut self,
        id: LintId,
        message: &str,
        item: &public_api::PublicItem,
        span: Option<&rustdoc_types::Span>,
    ) {
        if let Some(crate_item) = self.context.crate_.index.get(&item.id()) {
            let allow_id = format!("allow({})", id.qualified_name());
            if crate_item.attrs.iter().any(|attr| attr.contains(&allow_id)) {
                return;
            }
        }

        if !self.diagnostic_enabled(id) {
            return;
        }

        self.introduce_diagnostic(id);

        println!("⚠️ {message}");
        if let Some(span) = span {
            println!(
                "  at {}:{}",
                span.filename.as_path().display(),
                span.begin.0
            );
        }
        println!();
    }

    fn diagnostic_enabled(&self, id: LintId) -> bool {
        if self.cli.warn.iter().any(|s| *s == LintId::All || *s == id) {
            // explicitly enabled
            true
        } else {
            !self.cli.allow.iter().any(|s| *s == LintId::All || *s == id)
        }
    }

    fn introduce_diagnostic(&mut self, id: LintId) {
        if self.lints_output.contains(&id) {
            return;
        }

        self.lints_output.push(id);
        println!("------------------------------------------------------------");
        println!("{}", id.long_form_explanation());
        println!();
        println!(
            "Annotate this type with `#[cfg_attr(spal, allow({}))]` to ignore this lint",
            id.qualified_name()
        );
        println!("------------------------------------------------------------");
        println!();
    }
}

struct Context {
    crate_: rustdoc_types::Crate,
    public_api: public_api::PublicApi,
}

impl Context {
    fn find_trait(&self, path: &[&str]) -> Option<Id> {
        for (id, item_summary) in self.crate_.paths.iter() {
            if item_summary.kind == ItemKind::Trait && item_summary.path == path {
                return Some(*id);
            }
        }
        None
    }
}
