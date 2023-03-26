// Define a User struct which contains 2 fields: 
// name (string e.g "John")
// balance (tuple e.g (100.00, "SGD"))
struct User {
    name: String,
    balance: (f32, String),
}

// Define a User method (using impl) 
    // called print_user_detail, 
        // which simply prints the name, balance and currency of the user.
impl User {
    fn print_user_detail (&self) {
        println!("name {}, balance {}, currency {}", 
            self.name, self.balance.0, self.balance.1);
    }
}

// Define an accrue_interest function, which takes in a user and interest percentage as 2 separate parameters. 
    // Within the function, increase the users' balance by the interest percentage, and print out the user details by calling the print_user_detail method. 
fn accrue_interest (cur_user: &mut User, int_pct: f32) {
    cur_user.balance.0 = cur_user.balance.0 + cur_user.balance.0 * int_pct / 100.0;
    cur_user.print_user_detail();
}

fn main() {
    // In the main function, create a user variable of type User, populating the field values of name, and balance and currency.
    let mut new_user = User {
        name: "Farris".to_owned(),
        balance: (100.0, "USD".to_owned()),
    };
    
    // Then, call the accrue_interest function.
    // Bonus: After the call to accrue_interest, call it multiple times so that the user may benefit from compounding interest.
    for _element in 1..=3 {
        accrue_interest(&mut new_user, 7.0);
    }
}
    