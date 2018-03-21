# Rusty Snake

Rusty is a battle snake built with Rocket, a rust web developement framework. Rusty's strategy isn't clear yet but it will likely have multiple iterations with different strategies. Rusty purpose is to bring enjoyment to me and win battlesnake competitions. Rusty also helps me learn rust.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

In order to use and develope Rusty your going to need a few things:

1. [Rust](https://www.rust-lang.org/en-US/install.html)
2. [A very basic understanding of the web](https://code.tutsplus.com/tutorials/http-the-protocol-every-web-developer-must-know-part-1--net-31177)
3. [A understanding of the game this bot is trying to play](https://github.com/sendwithus/battlesnake-server/#readme)

### Installing

Once your done with the basics copy and paste the command below into your terminal:

```
git clone https://github.com/McRaeAlex/RustySnake.git
```

Then

```
cd RustySnake
```
and finally
```
cargo run
```
You should see something like this
```
Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 8
    => secret key: generated
    => limits: forms = 32KiB
    => tls: disabled
🛰  Mounting '/':
    => GET /
    => POST /start
🚀  Rocket has launched from http://localhost:8000
```
if you don't and it gives you errors saying missing features or something about nightly, run:
```
rustup default nightly
```
to switch your default toolchain to nightly

## Deployment (TODO)

Add additional notes about how to deploy this on a live system

## Built With

* [Rust](https://www.rust-lang.org/en-US/install.html) - The programming language used
* [Rocket](https://rocket.rs/) - The web framework used


## Contributing

This is more of a personal project than anything. However if you would like to contribute shoot me a email: alex@alexandermcrae.com 

## Authors

* **Alexander McRae** - *Everything so far* - [Alex McRae](http://www.alexandermcrae.com)

## License

This project does not have a license at this time

## Acknowledgments

* Rust for confusing me
* SendWithUs for making the competition