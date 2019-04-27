#[allow(dead_code)]
/*
Module system can be used to hierarchically split code in logical modules
and manage visibility between them.

A module is a collection of items: functions,
structs, traits, impl blocks, and even other modules

By default, tems in a module have private visibility, but this can be overriden
with the pub modifier. Only the public items of a module
can be accessed from outside the module scope.
*/

// Module named 'my_mod'
mod my_mod {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }

    }

    pub fn call_public_function_in_my_mod() {
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

pub fn run() {
    // Modules allow disambiguation between items that have the same name.
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the mode specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();

    // Error! `private_function` is private
    //my_mod::nested::private_function();

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
}

// ! Struct visibility
/*
Structs have an extra level of visiblity with their fields.
The visibility defaults to private, overridable with pub modifier.
Visiblity only matters when a struct is accessed from outside the
module where it is defined.

Public structs with public fields can be constructed as usual, i.e.
let open_box = my::OpenBox { contents: "public information"};
and their fields can be normally accessed, i.e.
println!("{}", open_box.contents);

Public structs with private fields cannot be constructed using field names.
However, they can be created using public constructors.
*/
