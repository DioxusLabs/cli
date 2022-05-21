use super::*;

#[derive(Clone, Debug, Parser)]
#[clap(name = "render")]
pub struct Render {
    /// Input file
    #[clap(short, long)]
    pub file: Option<String>,

    /// Output file, stdout if not present
    #[clap(parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// Input file
    #[clap(short, long)]
    pub source: Option<String>,
}

impl Render {
    pub fn render(self) -> Result<()> {
        let Render {
            output,
            file,
            source,
        } = self;

        let contents = match source {
            Some(input) => input,
            None => match file {
                Some(input) => std::fs::read_to_string(&input).unwrap_or_else(|e| {
                    log::error!("Cloud not read input file: {}.", e);
                    exit(0);
                }),
                None => {
                    if atty::is(atty::Stream::Stdin) {
                        log::error!("No input file, source, or stdin to translate from.");
                        exit(0);
                    }

                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer).unwrap();

                    buffer.trim().to_string()
                }
            },
        };

        let out_buf = rsx_parser::rsx_to_html(&contents)
            .map_err(|e| Error::ParseError(format!("{:?}", e)))?;

        if let Some(output) = output {
            std::fs::write(&output, out_buf)?;
        } else {
            print!("{}", out_buf);
        }

        Ok(())
    }
}
