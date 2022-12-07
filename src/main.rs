use console::{Emoji, style};
use xv6_fsck::{parser};
use xv6_fsck::fs::error::FSError;

static DISK: Emoji<'_, '_> = Emoji("💿", "");
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static ERROR: Emoji<'_, '_> = Emoji("❌ ", ":-(");

fn handle_results(results: &[Result<(), FSError>]) -> bool {
    let mut has_error = false;
    for result in results {
        if let Err(e) = result {
            has_error = true;
            eprintln!("{}: {}", style("error").bold().red(), style(e).bold());
        }
    }
    has_error
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

    let mut results = vec![];
    let mut has_error = false;

    /* Parse */
    println!(
        "{} {} Parsing xv6 filesystem image...",
        style("[1/5]").bold().dim(),
        DISK
    );
    let img = parser::read_img(path);
    let fs = parser::parse_fs(&img);

    /* Check superblock */
    println!(
        "{} {} Checking superblock...",
        style("[2/5]").bold().dim(),
        LOOKING_GLASS
    );
    results.push(fs.superblock.check_fields());
    has_error |= handle_results(&results);
    results.clear();

    /* Check block usage */
    println!(
        "{} {} Checking block usage...",
        style("[3/5]").bold().dim(),
        LOOKING_GLASS
    );
    results.push(fs.check_datablock_ref());
    // results.push(fs.check_bitmap());
    has_error |= handle_results(&results);
    results.clear();

    /* Check directory */
    println!(
        "{} {} Checking directory...",
        style("[4/5]").bold().dim(),
        LOOKING_GLASS
    );
    results.push(fs.check_current_directory());
    results.push(fs.check_parent_directory());
    has_error |= handle_results(&results);
    results.clear();

    /* Check inode */
    println!(
        "{} {} Checking inode...",
        style("[5/5]").bold().dim(),
        LOOKING_GLASS
    );
    results.push(fs.check_device_numbers());
    results.push(fs.check_nlink());
    results.push(fs.check_addrs_ref());
    // results.push(fs.check_addrs_len());
    has_error |= handle_results(&results);
    results.clear();

    if has_error {
        println!("{} {}", ERROR, style("Found errors").bold());
    } else {
        println!("{} {}", SPARKLE, style("No errors").bold());
    }
}
