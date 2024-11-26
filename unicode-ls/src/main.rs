use std::{collections::HashMap, process::Command};

use clap::Parser;
use simple_completion_language_server::*;
use snippets::Snippet;

macro_rules! create_snippet_map {
    ($($k:expr => $v:expr),*) => {{
        let mut v = vec![];
        $(
            v.push(Snippet {
                scope: None,
                prefix: $k.to_string(),
                description: Some($v.to_string().clone()),
                body: value,
            });
        )*
        v
    }};
}

fn get_prefix(s: &str) -> Option<String> {
    let s = s.replace("LATIN ", "");
    let s = s.replace("BALINESE ", "");
    let s = s.replace("GREEK ", "");
    let s = s.replace("TAI THAM HORA ", "");
    let s = s.replace("THAM COMBINING CRYPTOGRAMMIC ", "");
    let s = s.replace("TAI THAM SIGN ", "");
    let s = s.replace("TAI THAM VOWEL ", "");
    let s = s.replace(" ", "-");
    if s == "<control>" {
        return None;
    }

    Some(s)
}

#[derive(Parser)]
#[clap(version, long_about = None, about = "Unicode language server")]
struct Cli {
    #[arg(short, long)]
    include_all_symbols: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let mut snippets = create_snippet_map! {
        "Rightarrow" => '⇒',
        "=>" => '⇒',
        "rightarrow" => '→',
        "->" => '→',
        "supset" => '⊃',
        "Leftrightarrow" => '⇔',
        "<=>" => '⇔',
        "leftarrowarrow" => '↔',
        "<->" => '↔',
        "equiv" => '≡',
        "=" => '≡',
        "lnot" => '¬',
        "neg" => '¬',
        "!=" => '¬',
        "=" => '＝',
        "->" => "⇨",
        "_0" => '₀',
        "_1" => '₁',
        "_2" => "₂",
        "|>" => "▸",
        "<-" => '←',
        "<=" => '⇐',
        "^-1" => "⁻¹",
        "~~~" => '≋',
        "<|" => '◂',
        "2" => '𝟚',
        "^e" => 'ᵉ',
        "*" => '★',
        "wedge" => '∧',
        "^" => '∧',
        "land" => '∧',
        "*" => '·',
        "*" => 'º',
        "1/2" => '½',
        "1/4" => '¼',
        "3/4" => '¾',
        "x" => '×',
        "*" => '×',
        "o" => 'Ø',
        "empty" => 'Ø',
        "div" => '÷',
        "/" => '÷',
        "E" => 'Ɛ',
        "f" => 'ƒ',
        "w" => 'Ɯ',
        "lambda" => 'ƛ',
        "o" => 'Ɵ',
        "T" => 'Ƭ',
        "Y" => 'Ʊ',
        "V" => 'Ʋ',
        "Z" => 'Ƶ',
        "z" => 'ƶ',
        "3" => 'Ʒ',
        "E" => 'Ƹ',
        "e" => 'ƹ',
        "3" => 'ƺ',
        "|=" => 'ǂ',
        ":" => '⦂',
        "N" => 'ℕ',
        "C" => 'ℂ',
        "delta" => 'Δ',
        "contains" => '∋',
        "not contains" => '∌',
        "superset" => '⊃',
        "superset or equal" => '⊇',
        "union" => '∪',
        "not element of" => '∉',
        "element of" => '∈',
        "subset" => '⊂',
        "subset or equal" => '⊆',
        "there does not exists" => '∄',
        "intersection" => '∩',
        "intersect" => '∩',
        "!3" => '∌',
        "Q" => 'ℚ',
        "Z" => 'ℤ',
        "R" => 'ℝ',
        ";" => '⨾',
        "|->" => '↦',
        ">>" => '»',
        "cdot" => '·',
        "v" => '∨',
        "f" => '∫',
        "f-" => '∮',
        "ff" => '∬',
        "(" => '⟨',
        ")" => '⟩',
        "monad" => '⊙',
        "lor" => '∨',
        "vee" => '∨',
        "||" => '∥',
        "parallel" => '∥',
        "oplus" => '⊕',
        "veebar" => '⊻',
        "notequiv" => '≢',
        "!=" => '≢',
        "top" => '⊤',
        "T" => '⊤',
        "bot" => '⊥',
        "forall" => '∀',
        "A" => '∀',
        "E" => '∃',
        "exists" => '∃',
        "vdash" => '⊢',
        "turnstile" => '⊢',
        "|-" => '⊢',
        "vDash" => '⊨',
        "|=" => '⊨',
        "Leftrightarrow" => '⇔',
        "nvdash" => '⊬',
        "nvDash" => '⊭',
        "Box" => '□',
        "Diamond" => '◇',
        "therefore" => '∴',
        "because" => '∵',
        ":=" => '≔',
        "alpha" => 'α',
        "a" => 'α',
        "beta" => 'β',
        "b" => 'β',
        "B" => 'β',
        "y" => 'γ',
        "Y" => 'γ',
        "gamma" => 'γ',
        "Gamma" => 'Γ',
        "delta" => 'δ',
        "Delta" => 'Δ',
        "epsilon" => 'δ',
        "zeta" => 'ζ',
        "eta" => 'η',
        "n" => 'η',
        "theta" => 'θ',
        "Theta" => 'Θ',
        "iota" => 'ι',
        "kappa" => 'κ',
        "k" => 'κ',
        "\\" => 'λ',
        "lambda" => 'λ',
        "Lambda" => 'Λ',
        "^" => 'Λ',
        "mu" => 'μ',
        "u" => 'μ',
        "E" => 'ξ',
        "xi" => 'ξ',
        "===" => 'Ξ',
        "Xi" => 'Ξ',
        "pi" => 'π',
        "Pi" => 'Π',
        "rho" => 'ρ',
        "sigma" => 'σ',
        "Sigma" => 'Σ',
        "tau" => 'τ',
        "t" => 'τ',
        "upsilon" => 'υ',
        "u" => 'υ',
        "phi" => 'φ',
        "Phi" => 'Φ',
        "chi" => 'χ',
        "x" => 'χ',
        "psi" => 'ψ',
        "Psi" => 'Ψ',
        "omega" => 'ω',
        "Omega" => 'Ω',
        "^~~" => " ͌",
        "^*/*" => " ͋",
        "^!~" => " ͊",
        "_=" => " ͇",
        "^=" => " ̿",
        "^x" => " ̽",
        "^*" => " ̽",
        "_-" => " ̲",
        "_T" => " ̞",
        "_+" => " ̟",
        "_bot" => " ̞",
        "_|-" => " ̙",
        "^o" => " ̊"
    };

    dbg!(cli.include_all_symbols);

    if cli.include_all_symbols {
        for line in include_str!("data.txt").split("\n") {
            if line.is_empty() {
                continue;
            }
            let line = line.split(";").collect::<Vec<_>>();
            let [c, alias, ..] = line.as_slice() else {
                continue;
            };

            let Ok(c) = u32::from_str_radix(c, 16) else {
                continue;
            };

            let Ok(c) = char::try_from(c) else {
                continue;
            };

            let alias = alias.to_lowercase();
            let Some(prefix) = get_prefix(&alias) else {
                continue;
            };

            snippets.push(Snippet {
                scope: None,
                prefix,
                description: Some(format!("{c}")),
                body: format!("{c}"),
            });
        }
    }

    server::start(
        stdin,
        stdout,
        snippets
            .into_iter()
            .filter(|s| {
                !s.body.is_empty()
                    && match &s.description {
                        Some(s) => !s.is_empty(),
                        None => false,
                    }
            })
            .collect(),
        HashMap::new(),
        etcetera::home_dir().unwrap().to_str().unwrap().into(),
    )
    .await;
}
