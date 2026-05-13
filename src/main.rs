use clap::{Parser, Subcommand};
use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;
use wicket::{StandingClass, SurfaceVerdict};
use wicket_guard::{cook_diff, parse, CookOpts};

#[derive(Parser)]
#[command(
    name = "wicket-preflight",
    about = "Admissibility preflight for AI-agent-authored diffs."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Check a unified diff for unauthorized mutations to authority-bearing surfaces.
    Check {
        /// Path to a unified-diff file. Use `-` for stdin.
        #[arg(long)]
        diff: PathBuf,
        /// One-sentence rule the actor cites for the change.
        #[arg(long)]
        because: String,
        /// Actor identifier.
        #[arg(long, default_value = "claude-code")]
        actor: String,
        /// Actor's standing class.
        #[arg(long, default_value = "execute")]
        standing: String,
        /// Emit JSON instead of a human-readable summary.
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Check {
            diff,
            because,
            actor,
            standing,
            json,
        } => run_check(diff, because, actor, standing, json),
    }
}

fn run_check(
    diff_path: PathBuf,
    because: String,
    actor: String,
    standing: String,
    json: bool,
) -> ExitCode {
    let body = match diff_path.to_str() {
        Some("-") => {
            let mut buf = String::new();
            if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
                eprintln!("error: read stdin: {e}");
                return ExitCode::from(64);
            }
            buf
        }
        _ => match std::fs::read_to_string(&diff_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: read {}: {e}", diff_path.display());
                return ExitCode::from(64);
            }
        },
    };

    let standing_class = match parse_standing(&standing) {
        Some(s) => s,
        None => {
            eprintln!("error: unknown --standing '{standing}'");
            return ExitCode::from(64);
        }
    };

    let parsed = parse(&body);
    let opts = CookOpts {
        actor: &actor,
        standing: standing_class,
        because: &because,
    };
    let cooked = cook_diff(&parsed, &opts);

    if cooked.is_empty() {
        if json {
            println!("{{\"verdicts\":[]}}");
        } else {
            println!("OK: no authority-bearing surfaces touched.");
        }
        return ExitCode::SUCCESS;
    }

    let mut worst = SurfaceVerdict::Authorized;
    let mut json_verdicts: Vec<serde_json::Value> = Vec::new();

    for ci in &cooked {
        let outcome = wicket::check(&ci.intent);
        worst = worst_of(worst, outcome.surface_verdict);
        if json {
            json_verdicts.push(serde_json::json!({
                "target": ci.intent.target,
                "intended_action": ci.intent.intended_action,
                "surface_verdict": snake(outcome.surface_verdict),
                "reason_codes": outcome.reason_codes,
                "receipt_id": outcome.receipt.receipt_id,
            }));
        } else {
            println!(
                "{:>11}  {}  ({})",
                snake(outcome.surface_verdict),
                ci.intent.target,
                ci.intent.intended_action,
            );
            for code in &outcome.reason_codes {
                println!("             - {code}");
            }
            println!("             receipt: {}", outcome.receipt.receipt_id);
        }
    }

    if json {
        let body = serde_json::json!({ "verdicts": json_verdicts });
        println!("{}", serde_json::to_string_pretty(&body).unwrap());
    }

    match worst {
        SurfaceVerdict::Authorized | SurfaceVerdict::AdvisoryOnly => ExitCode::SUCCESS,
        SurfaceVerdict::Unaccounted => ExitCode::from(2),
        _ => ExitCode::from(1),
    }
}

fn parse_standing(s: &str) -> Option<StandingClass> {
    Some(match s {
        "observe" => StandingClass::Observe,
        "interpret" => StandingClass::Interpret,
        "recommend" => StandingClass::Recommend,
        "authorize" => StandingClass::Authorize,
        "execute" => StandingClass::Execute,
        _ => return None,
    })
}

fn snake(v: SurfaceVerdict) -> &'static str {
    match v {
        SurfaceVerdict::Authorized => "authorized",
        SurfaceVerdict::AdvisoryOnly => "advisory",
        SurfaceVerdict::Denied => "denied",
        SurfaceVerdict::Gap => "gap",
        SurfaceVerdict::Unaccounted => "unaccounted",
    }
}

fn worst_of(a: SurfaceVerdict, b: SurfaceVerdict) -> SurfaceVerdict {
    fn rank(v: SurfaceVerdict) -> u8 {
        match v {
            SurfaceVerdict::Authorized => 0,
            SurfaceVerdict::AdvisoryOnly => 1,
            SurfaceVerdict::Gap => 2,
            SurfaceVerdict::Denied => 3,
            SurfaceVerdict::Unaccounted => 4,
        }
    }
    if rank(b) > rank(a) {
        b
    } else {
        a
    }
}
