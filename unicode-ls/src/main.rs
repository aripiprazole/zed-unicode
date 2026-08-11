use std::collections::HashMap;

use clap::Parser;
use simple_completion_language_server::*;
use snippets::Snippet;

macro_rules! create_snippet_map {
    ($($k:expr => $v:expr),*) => {{
        let mut v = vec![];
        let mut h = vec![];
        $(
            if !h.contains(&$k.to_string()) {
                v.push(Snippet {
                    scope: None,
                    prefix: $k.to_string(),
                    description: Some($v.to_string().clone()),
                    body: $v.to_string(),
                });
                h.push($k.to_string());
            }
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
    /// Require this string before a snippet trigger, e.g. "u" turns "lambda"
    /// into "ulambda". Use an alphanumeric prefix, not punctuation.
    #[arg(long, default_value = "")]
    prefix: String,
}

fn build_snippets(cli: &Cli) -> Vec<Snippet> {
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
        "approx" => '≈',
        "~~" => '≈',
        "~~~" => '≋',
        "<|" => '◂',
        "2" => '𝟚',
        "^e" => 'ᵉ',
        "*" => '★',
        "wedge" => '∧',
        "^" => '∧',
        "land" => '∧',
        "*" => '·',
        "^*" => 'º',
        "1/2" => '½',
        "1/4" => '¼',
        "3/4" => '¾',
        "x" => '×',
        "o/" => 'Ø',
        "empty" => 'Ø',
        "div" => '÷',
        "/" => '÷',
        "E" => 'Ɛ',
        "f" => 'ƒ',
        "W" => 'Ɯ',
        "o" => 'Ɵ',
        "T" => 'Ƭ',
        "Y" => 'Ʊ',
        "V" => 'Ʋ',
        "Z/" => 'Ƶ',
        "z/" => 'ƶ',
        "3" => 'Ʒ',
        "E" => 'Ƹ',
        "e" => 'ƹ',
        "3" => 'ƺ',
        "|=" => 'ǂ',
        ":" => '⦂',
        "N" => 'ℕ',
        "C" => 'ℂ',
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
        "open parenthesis" => '⟨',
        "(" => '⟨',
        ")" => '⟩',
        "close parenthesis" => '⟩',
        "monad" => '⊙',
        "lor" => '∨',
        "vee" => '∨',
        "||" => '∥',
        "parallel" => '∥',
        "oplus" => '⊕',
        "veebar" => '⊻',
        "not equiv" => '≢',
        "!=" => '≢',
        "top" => '⊤',
        "T" => '⊤',
        "bot" => '⊥',
        "forall" => '∀',
        "A" => '∀',
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
        "Alpha" => 'Α',
        "beta" => 'β',
        "b" => 'β',
        "B" => 'β',
        "Beta" => 'Β',
        "y" => 'γ',
        "Y" => 'γ',
        "gamma" => 'γ',
        "Gamma" => 'Γ',
        "delta" => 'δ',
        "Delta" => 'Δ',
        "epsilon" => 'ε',
        "Epsilon" => 'Ε',
        "zeta" => 'ζ',
        "Zeta" => 'Ζ',
        "eta" => 'η',
        "n" => 'η',
        "Eta" => 'Η',
        "theta" => 'θ',
        "Theta" => 'Θ',
        "iota" => 'ι',
        "Iota" => 'Ι',
        "kappa" => 'κ',
        "k" => 'κ',
        "Kappa" => 'Κ',
        "\\" => 'λ',
        "lambda" => 'λ',
        "Lambda" => 'Λ',
        "^" => 'Λ',
        "mu" => 'μ',
        "Mu" => 'Μ',
        "nu" => 'ν',
        "Nu" => 'Ν',
        "E" => 'ξ',
        "xi" => 'ξ',
        "===" => 'Ξ',
        "Xi" => 'Ξ',
        "omicron" => 'ο',
        "Omicron" => 'Ο',
        "pi" => 'π',
        "Pi" => 'Π',
        "rho" => 'ρ',
        "Rho" => 'Ρ',
        "sigma" => 'σ',
        "Sigma" => 'Σ',
        "tau" => 'τ',
        "t" => 'τ',
        "Tau" => 'Τ',
        "upsilon" => 'υ',
        "u" => 'υ',
        "Upsilon" => 'Υ',
        "phi" => 'φ',
        "Phi" => 'Φ',
        "chi" => 'χ',
        "x" => 'χ',
        "Chi" => 'Χ',
        "psi" => 'ψ',
        "Psi" => 'Ψ',
        "omega" => 'ω',
        "Omega" => 'Ω',
        "->>" => '↠'
    };

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

    snippets
        .into_iter()
        .filter(|s| {
            !s.body.is_empty()
                && match &s.description {
                    Some(s) => !s.is_empty(),
                    None => false,
                }
        })
        .map(|mut s| {
            if !cli.prefix.is_empty() {
                s.prefix = format!("{}{}", cli.prefix, s.prefix);
            }
            s
        })
        .collect()
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let all_snippets = build_snippets(&cli);

    server::start(
        stdin,
        stdout,
        all_snippets,
        HashMap::new(),
        etcetera::home_dir().unwrap().to_str().unwrap().into(),
    )
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn cli(prefix: &str) -> Cli {
        Cli {
            include_all_symbols: false,
            prefix: prefix.to_string(),
        }
    }

    fn find<'a>(snippets: &'a [Snippet], prefix: &str) -> &'a Snippet {
        snippets
            .iter()
            .find(|s| s.prefix == prefix)
            .unwrap_or_else(|| panic!("no snippet with prefix {prefix:?}"))
    }

    #[test]
    fn get_prefix_strips_script_names_and_spaces() {
        assert_eq!(
            get_prefix("GREEK SMALL LETTER ALPHA"),
            Some("SMALL-LETTER-ALPHA".to_string())
        );
        assert_eq!(get_prefix("<control>"), None);
    }

    #[test]
    fn no_duplicate_prefixes_in_base_map() {
        let snippets = build_snippets(&cli(""));
        let mut seen = HashSet::new();
        for s in &snippets {
            assert!(seen.insert(&s.prefix), "duplicate prefix: {}", s.prefix);
        }
    }

    #[test]
    fn lambda_defaults_to_greek_letter() {
        let snippets = build_snippets(&cli(""));
        assert_eq!(find(&snippets, "lambda").body, "λ");
    }

    #[test]
    fn all_greek_letters_are_present() {
        let snippets = build_snippets(&cli(""));
        let lower = [
            "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota",
            "kappa", "lambda", "mu", "nu", "xi", "omicron", "pi", "rho", "sigma", "tau",
            "upsilon", "phi", "chi", "psi", "omega",
        ];
        let upper = [
            "Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Eta", "Theta", "Iota",
            "Kappa", "Lambda", "Mu", "Nu", "Xi", "Omicron", "Pi", "Rho", "Sigma", "Tau",
            "Upsilon", "Phi", "Chi", "Psi", "Omega",
        ];
        for name in lower.into_iter().chain(upper) {
            find(&snippets, name);
        }
    }

    #[test]
    fn prefix_flag_is_prepended_to_every_snippet() {
        let snippets = build_snippets(&cli("u"));
        assert_eq!(find(&snippets, "ulambda").body, "λ");
        assert!(snippets.iter().all(|s| s.prefix.starts_with('u')));
    }
}
