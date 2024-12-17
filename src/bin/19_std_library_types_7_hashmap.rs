mod hash_map {
    use std::collections::HashMap;

    #[derive(PartialEq, Eq, Hash)]
    struct Account<'a> {
        username: &'a str,
        password: &'a str,
    }

    struct AccountInfo<'a> {
        name: &'a str,
        email: &'a str,
    }

    type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

    fn try_logon(accounts: &Accounts, username: &str, password: &str) {
        println!("Username: {}", username);
        println!("Password: {}", password);
        println!("Attempting logon...");

        let logon = Account { username, password };

        match accounts.get(&logon) {
            Some(account_info) => {
                println!("Successful logon!");
                println!("Name: {}", account_info.name);
                println!("Email: {}", account_info.email);
            }
            None => println!("Login failed!"),
        }
    }

    pub fn test() {
        let mut accounts = HashMap::new();

        let account = Account {
            username: "j.everyman",
            password: "password123",
        };

        let account_info = AccountInfo {
            name: "John Everyman",
            email: "j.everyman@email.com",
        };

        accounts.insert(account, account_info);

        try_logon(&accounts, "j.everyman", "psasword123");

        try_logon(&accounts, "j.everyman", "password123");
    }
}

mod hash_set {
    use std::collections::HashSet;

    pub fn test() {
        let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
        let mut b = vec![2, 3, 4].into_iter().collect::<HashSet<_>>();

        assert!(a.insert(4));
        assert!(a.contains(&4));

        b.insert(5);

        println!("A: {:?}", a);
        println!("B: {:?}", b);

        println!("Union: {:?}", a.union(&b));
        println!("Difference: {:?}", a.difference(&b));
        println!("Intersection: {:?}", a.intersection(&b));
        println!("Symmetric Difference: {:?}", a.symmetric_difference(&b));
    }
}

fn main() {
    hash_map::test();
    println!();
    hash_set::test();
}
