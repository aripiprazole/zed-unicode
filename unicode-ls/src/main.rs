use std::collections::HashMap;

use simple_completion_language_server::*;
use snippets::Snippet;

macro_rules! create_snippet_map {
    ($($k:expr => $v:expr),*) => {{
        let mut m = std::collections::HashMap::new();
        $(m.insert($k.to_string(), format!("{}", $v)));*;
        m
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

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let mut snippets = vec![];
    let unicode: HashMap<String, String> = create_snippet_map! {
        "Rightarrow" => '⇒',
        "rightarrow" => '→',
        "supset" => '⊃',
        "Leftrightarrow" => '⇔',
        "leftarrowarrow" => '↔',
        "equiv" => '≡',
        "lnot" => '¬',
        "neg" => '¬',
        "wedge" => '∧',
        "land" => '∧',
        "cdot" => '·',
        "lor" => '∨',
        "vee" => '∨',
        "parallel" => '∥',
        "oplus" => '⊕',
        "veebar" => '⊻',
        "notequiv" => '≢',
        "top" => '⊤',
        "bot" => '⊥',
        "forall" => '∀',
        "exists" => '∃',
        "vdash" => '⊢',
        "turnstile" => '⊢',
        "vDash" => '⊨',
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
        "gamma" => 'γ',
        "Gamma" => 'Γ',
        "delta" => 'δ',
        "Delta" => 'Δ',
        "epsilon" => 'δ',
        "zeta" => 'ζ',
        "eta" => 'η',
        "theta" => 'θ',
        "Theta" => 'Θ',
        "iota" => 'ι',
        "kappa" => 'κ',
        "k" => 'κ',
        "lambda" => 'λ',
        "Lambda" => 'Λ',
        "mu" => 'μ',
        "xi" => 'ξ',
        "Xi" => 'Ξ',
        "pi" => 'π',
        "Pi" => 'Π',
        "rho" => 'ρ',
        "sigma" => 'σ',
        "Sigma" => 'Σ',
        "tau" => 'τ',
        "upsilon" => 'υ',
        "phi" => 'φ',
        "Phi" => 'Φ',
        "chi" => 'χ',
        "psi" => 'ψ',
        "Psi" => 'Ψ',
        "omega" => 'ω',
        "Omega" => 'Ω'
    };
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

    for (name, value) in unicode {
        snippets.push(Snippet {
            scope: None,
            prefix: name.clone(),
            description: Some(value.clone()),
            body: value,
        });
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
