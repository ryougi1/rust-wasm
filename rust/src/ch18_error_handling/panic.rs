/*
As a rule of tumb, an explicit panic is mainly useful for tests and dealing
with unrecoverably errors. 

The Option type is for when a value is optonal or when the lack of a value
is not an error condition. When dealing with Option, unwrap is fine for
prototyping and cases where it's absolutely certain that there is guaranteed
to be a vlaue. However, expect is more useful since it lets you specify an
error message.

When there is a chance that things do go wrong and the caller has to deal
with the problem, use Result. You can unwrap and expect them as well. 
*/

fn give_princess(gift: &str) {
    // Princesses hate snakes, so we need to stop if she disapproves!
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

pub fn run() {
    give_princess("teddy bear");
    give_princess("snake");
}