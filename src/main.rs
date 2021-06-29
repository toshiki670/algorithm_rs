use proconio::input;
mod n_triangle;

fn main() {
    println!("Select algorithms:");
    println!("0: n_triangle");

    println!("\nEnter number from 0 to 0:");
    input! {
        selected_num: u8,
    }

    println!();

    match selected_num {
        0 => n_triangle::exec(),
        1_u8..=u8::MAX => println!("Do nothing..."),
    }

    println!("\nGood bye!");
}
