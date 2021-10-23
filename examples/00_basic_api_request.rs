use serde::Deserialize;  // You 'install' a dependency by adding to Cargo.toml
use reqwest::Error;
use reqwest::header::USER_AGENT;

// derive is an attribute/macro (think preprocessor macro), it helps
// automagically create types/structs for things you just
// said you were using? Adds them to the following struct definition?
// so this appends the required fields for each of those?
// The Deserialize one requires you do a more involved 'install' meaning
// when you added serde to the Cargo.toml file, you have to give
// it an additional instruction to include some optional feature.
// the 'features = ["derive"]' part. Also need to do this
// for the other ones, check the toml. 
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

// another attribute, this one is used by the tokio async lib
// stuff, need to learn more. 
// we can declare the output type of main, its a Result
// with some <stuff, possible errors> in it?
#[tokio::main]
async fn main() -> Result<(), Error> {

    // Oh nice, string templating is pretty awesome. 
    // should read about format!()
    // Ok so the ! is used to differentiate a macro from a function
    // anything ending with a ! is a MACRO, functions don't have it
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "wpower12",
                              repo = "jsBEEBC");

    // println! expects a string literal, treats it as a kind of
    // format string? And then you pass some number of refs to 
    // the variables, it does the borrowing for you?
    println!("{}", request_url);


    // I had to change this from the tutorial. Some sites/ISP combos
    // mean you need a UserAgent value in the header of the 
    // request, or you get 403'd. So to deal with that we add 
    // a header for a UA. That means 'use'-ing another part 
    // of the reqwest crate. Same overall pattern, we make a
    // call by invoking a method on the client, but then the body
    // of the query can be built by chaining method calls. End
    // with an await? because these return Futures and we want to 
    // block. 
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url) // Note, this is a method, so you send a 'borrowed'
                           // reference to the actual variable. 
        .header(USER_AGENT, "USER THIS AGENT MOTHER ****ERS")
        .send()
        .await?;

    // We can deserialize that reponse's body json into into a 
    // variable of 'type' Vec<Users>, because we know that
    // the json contains a list of 0+ objects that match 
    // the structure we defined above for User. 
    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);


    // Since we declared the function to return a Result, we have
    // to return one of the possible types. One way to signal "No Error"
    // is with Ok(), we pass it an empty 'result' because we did the thing. 
    // The other paths through the code could end in exceptions during the
    // running of the other code, which might return Error instead. 
    // I think? - Note - The Results 'inner types' that its composed of are 
    // () and Error. It was an "OK" result, so we pass the OK type; ()?
    // Again, i THINK. 
    Ok(())
}

