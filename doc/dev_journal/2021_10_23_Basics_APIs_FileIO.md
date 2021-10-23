# 2021-10-23 - Initial Notes for Learning Rust

## Goals

### Learn Rust
So I want to learn rust! Making a simple-but-hopefully-useful-for-me app seemed the best way. I would like to make something 'start to finish'. That is, something that follows best practices from initial code organization and build structure, all the way to 'releasing' a finished binary that can be downloaded with the cargo commmand and ran on any POSIX system.

### Build SEPTA App
To have a motivating goal, I'd like to make a really simple CLI app that just tells you when the next SEPTA train for a line is coming to a station.

I'd like to keep it "unix"-y? Meaning I'd like to just read a config file that tells you what line and what station and running 'trains XX' would tell you the next trains for that config line. So basically recreating a simple version of the SEPTA app's favorites. 

So that means knowing how to do an API call, which means async stuff. Then also reading and writing to files. I'd like to make sure its all handled in a best practice manner, even if it gets bulkier because of that. 

I think there's basically three usages to consider;

* Config Content - 2 Parts?
	- In the config, you would save "SEPTA" as a line, storing the template for the API call
	- You would then have favorites that store a triple of line, station, direction.
* Line Template
	- Would need a couple of these? To start just the simplest one; for a set of trains
	- Then in the future one to get an actual schedule and cache it?

Lets just describe a minimum viable product;

* Assume its just for SEPTA, so the config file will just track favorites.	
	- Stretch goal is to add different lines, as above.
* Only do 'online' mode, so its not saving a cache or anything, just calls the API for you each time.
	- Stretch goal is to cache so it works offline. 

## Basics and API Request
I have a file in the  `doc/dev` directory that contains my work on this, the `00_basic_api_request.rs` file. This has an example script that uses a few crates to make an API request to github to find the stargazers for a repo. It shows off a lot of the basics of rust, as well as some intermediate stuff, like an intro to macros, using define, async, etc. This is pretty much a copy/paste of the example from the doc, with the added use of an actual client, instead of using the helper get macro. The example from the rust book is [here](https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html).

This is also an example of how we configure dependencies. It's not npm, you're not telling a thing to install from the command line, you are updating the .toml file. Luckily when you find a crate you want to use on the cargod site, it usually has a 'copy paste this' button. Also, be aware of what `modules` a crate exposes. Cargo doesnt include all of those by default, you may have to add a 'features' option in the toml file. Examples in the 00 script. To figure out what modules a given crate exposes, the best/most consistent thing to do is to go to the actual source for the crate. Within the `Cargo.toml` file for a crate is a section that lists out all the modules you could use. I think these are always in a `features` section of a crates toml file. Though maybe its also the members section? For reqwests its in the `features` section.

The next steps should be to handle file IO. Then we'd move onto more 'architecture'y questions like "how do you install rust binaries?" and if there are parts of that lifecycle that we'd use to create an initial config file. 

## File IO
Ok lets find a file io example and play with it. Just mark it up the same way, tweak it to prove you know what levers do what. 

I'm just deleting everything in the main.rs file. I'd like to figure out how to do my own organization soon, so I can invoke code from the examples directory from a main, instead of doing this. For later, this is working fine for now. 

I guess I could look more into cargo and create new build rules, but also, for later. 

First up, we just copied [this](https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html) tutorial, and noted it up. I'd like to extend this to also append something to the file. 

Ok so I combined that tutorial with a few things, the OpenOptions stuff from [this](https://stackoverflow.com/questions/30684624/what-is-the-best-variant-for-appending-a-new-line-in-a-text-file) SO answer. That kinda goes over how we open a file so we can append/write to it. Also, it takes some content from the 'create' example from the rust book [here](https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html).

Next steps would be to get rid of the RY and make a function that reads and prints the file to get rid of my copy paste bit. 

I added a function that does the above. I made the parameter be a BORROWED REFERENCE to a path. And that seems to work? Neat. 

This is cool. 

The match statement is awesome. 

The finished example is now in `02_basic_file_io.rs` in the `examples` directory. 

Next steps: get some responses from the actual SEPTA API.

## SEPTA API
The SEPTA API homepage is [here](http://www3.septa.org/api/). This includes a call out to the actual station/next-train endpoint.

An example call to that is: `http://www3.septa.org/hackathon/Arrivals/Suburban%20Station/5/`

So you give the `http://www3.septa.org/hackathon/Arrivals/` a station name, and a number of 'next trains' arriving.

Will need to figure out how to turn that into 'line info'. Meaning, we know what station we're leaving from, we need to find the next train on the right line for the rest of it. 

So this is likely schedule info, which might be on another endpoint? idk. 
