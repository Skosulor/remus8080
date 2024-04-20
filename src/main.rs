use remus8080::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options
{
    #[structopt(short = "r", long = "rom", help = "Path to ROM file")]
    rom: String,

    #[structopt( short = "f", long = "freq", help = "CPU frequency in Hz", default_value = "2000000")]
    cpu_freq: u32,
}

fn main() 
{
    let args = Options::from_args();
    let mut p = i8080::Processor::from_file(args.rom, args.cpu_freq);
    let mut dgb = debugger::Debugger::default();

    dgb.execute(&mut p, true);
    loop
    {
        match dgb.execute(&mut p, false)
        {
            Some(_) => (),
            None    => break,
        }
    }
}

