use console::{style, Emoji};
use xv6_fsck::fs::error::FSError;
use xv6_fsck::parser;

static DISK: Emoji<'_, '_> = Emoji("💿", "");
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨", ":-)");
static ERROR: Emoji<'_, '_> = Emoji("❌", ":-(");

fn handle_errors(errors: &[FSError]) {
    for e in errors {
        eprintln!("{}: {}", style("error").bold().red(), style(e).bold());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

    let mut errors = vec![];
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
    errors.append(&mut fs.superblock.check_fields());
    has_error |= !errors.is_empty();
    handle_errors(&errors);
    errors.clear();

    /* Check block usage */
    println!(
        "{} {} Checking block usage...",
        style("[3/5]").bold().dim(),
        LOOKING_GLASS
    );
    errors.append(&mut fs.check_datablock_ref());
    // errors.append(&mut fs.check_bitmap());
    has_error |= !errors.is_empty();
    handle_errors(&errors);
    errors.clear();

    /* Check directory */
    println!(
        "{} {} Checking directory...",
        style("[4/5]").bold().dim(),
        LOOKING_GLASS
    );
    errors.append(&mut fs.check_current_directory());
    errors.append(&mut fs.check_parent_directory());
    has_error |= !errors.is_empty();
    handle_errors(&errors);
    errors.clear();

    /* Check inode */
    println!(
        "{} {} Checking inode...",
        style("[5/5]").bold().dim(),
        LOOKING_GLASS
    );
    errors.append(&mut fs.check_device_numbers());
    errors.append(&mut fs.check_nlink());
    errors.append(&mut fs.check_addrs_ref());
    // errors.append(&mut fs.check_addrs_len());
    has_error |= !errors.is_empty();
    handle_errors(&errors);
    errors.clear();

    if has_error {
        println!("{}  {}", ERROR, style("Found errors").bold());
    } else {
        println!("{}  {}", SPARKLE, style("No errors").bold());
    }
}
