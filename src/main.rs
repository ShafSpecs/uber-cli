use clap::Parser;

mod coords;
mod drivers;
mod rides;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Commands {
    #[arg(short, long)]
    book: String,
}

fn main() {
    let mut vector: Vec<String> = Vec::new();
    // drivers::view_drivers();
    let current_coords = coords::get_coords(String::from("296a jide oki")).ok().unwrap();

    vector.push(current_coords[0].clone());
    vector.push(current_coords[1].clone());

    rides::get_ride_eta(vector);
}
