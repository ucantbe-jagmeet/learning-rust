//Rust has many ways to allow some safe mutability inside of something that is immutable. The most simple way is called Cell.
//A RefCell is another way to change values without needing to declare mut. It means "reference cell", and is like a Cell but uses references instead of copies.
use std::cell::{Cell, RefCell};
use std::sync::Mutex;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

fn main() {
    /*
      let super_phone_3000 = PhoneModel {
         company_name: "YY Electronics".to_string(),
         model_name: "Super Phone 3000".to_string(),
         screen_size: 7.5,
         memory: 4_000_000,
         date_issued: 2020,
         on_sale: Cell::new(true),
     };

     // 10 years later, super_phone_3000 is not on sale anymore
     super_phone_3000.on_sale.set(false);

     let user_1 = User {
         id: 1,
         year_registered: 2020,
         username: "User 1".to_string(),
         active: RefCell::new(true),
     };

     println!("{:?}", user_1.active);
    */

    /*
        There are many methods for RefCell. Two of them are .borrow() and .borrow_mut(). With these methods, you can do the same thing you do with & and &mut. The rules are the same:

    Many borrows is fine,
    one mutable borrow is fine,
    but mutable and immutable together is not fine.

         */

    //######## Mutex ##############
    /*
      let my_mutex = Mutex::new(5); // A new Mutex<i32>. We don't need to say mut
    let mut mutex_changer = my_mutex.lock().unwrap(); // mutex_changer is a MutexGuard
                                                      // It has to be mut because we will change it
                                                      // Now it has access to the Mutex
                                                      // Let's print my_mutex to see:

    println!("{:?}", my_mutex); // This prints "Mutex { data: <locked> }"
                                // So we can't access the data with my_mutex now,
                                // only with mutex_changer

    println!("{:?}", mutex_changer); // This prints 5. Let's change it to 6.

    *mutex_changer = 6; // mutex_changer is a MutexGuard<i32> so we use * to change the i32

    println!("{:?}", mutex_changer);
      */
    //But mutex_changer still has a lock after it is done. How do we stop it? A Mutex is unlocked when the MutexGuard goes out of scope. "Go out of scope" means the code block is finished. For example:
    /*
       let my_mutex = Mutex::new(5);
    {
        let mut mutex_changer = my_mutex.lock().unwrap();
        *mutex_changer = 6;
    } // mutex_changer goes out of scope - now it is gone. It is not locked anymore

    println!("{:?}", my_mutex); // Now it says: Mutex { data: 6 }
     */

    //If you don't want to use a different {} code block, you can use std::mem::drop(mutex_changer). std::mem::drop means "make this go out of scope"

    /*

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    *mutex_changer = 6;
    std::mem::drop(mutex_changer); // drop mutex_changer - it is gone now
                                   // and my_mutex is unlocked

    println!("{:?}", my_mutex); // Now it says: Mutex { data: 6 }
    */
}
