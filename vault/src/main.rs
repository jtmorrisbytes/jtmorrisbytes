use std::io::Write;

use clap::Parser;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use zeroize::Zeroize;

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Command {
    #[command(about = "Initializes the vault by generating passkeys and encrypted structure")]
    Initialize {
        #[arg(long)]
        with_existing_bips: bool,
    },
}

#[derive(clap::Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

// fn initialize_vault()

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut args = std::env::args();
    // let _ = args.next().ok_or("")?;
    // let verb = args.next().ok_or("")?;

    let args = Cli::parse();

    match args.command {
        Command::Initialize { with_existing_bips } => {
            // if the user already has a bips39 passphrase
            println!("This function is not complete yet. expect bugs");

            let bips = {
                if with_existing_bips {
                    let mut phrases = vec![];
                    // collect the bips passphrase from secure method
                    vault::term::prompt_user_for_bips(&mut phrases)?;
                    println!("The program has recieved your bips 39 passcode");
                    phrases
                } else {
                    // generate bips
                    // let next = args.next().unwrap_or("bips")

                    println!("This program will generate a bips 39 passphrase. please wait");
                    let mut passphrases = vault::bips::bips_39()?;
                    let qrcode = qrcodegen::QrCode::encode_text(
                        &passphrases.join(" "),
                        qrcodegen::QrCodeEcc::High,
                    )?;
                    // let html = vault::graphics::render_bips39_phrases_to_html(passphrases)?;
                    // std::fs::File::create("bips39.html")?.write_all(html.as_bytes())?;
                    vault::graphics::render_qrcode_to_console(&qrcode);
                    println!(
                        "Here is your BIPS39 QR code. Make sure you scan this code and save it somewhere.\n Press enter to continue"
                    );
                    let _ = std::io::stdout().flush().ok();
                    std::io::stdin().read_line(&mut String::new())?;

                    println!("Your BIPS39 recovery phrase is");
                    for (i, phrase) in passphrases.iter().enumerate() {
                        if i % 3 == 0 && i > 0 {
                            println!("{phrase}")
                        } else {
                            print!("{phrase} ")
                        }
                    }
                    println!();

                    let mut line = String::new();
                    loop {
                        println!(
                            "Do you want to print a backup copy?\n If Yes, the program will attempt to print to the default printer.\n\
                                Printer selection is not supported at this time.\nPlease type Y or N and press enter"
                        );
                        std::io::stdin().read_line(&mut line)?;
                        if line == "Y" || line == "N" {
                            break;
                        }
                        println!("Please enter yes or no")
                    }
                    if line == "Y" {
                        println!(
                            "the program is attempting to send the job to the requested printer"
                        );
                        vault::print::win32_print_bip39_using_gdi(&qrcode, &passphrases)?;
                    }
                    let confirmation = String::new();
                    while confirmation != "Y" {
                        println!("To confirm that you have your passkey, press Y and then ENTER. If you lose you passkey, \n\
                        The app will not be able to decrypt secrets and all vault data will be lost.\n
                        If you do not have your passkey, press N an and then enter and run the command again");
                        std::io::stdin().read_line(&mut line)?;
                    }
                    passphrases
                }
            };

            // we will now verify the bips

            Ok(())
        }
        _ => Err("".into()),
    }
}

