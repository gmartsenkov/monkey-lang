use monkey_lang::repl;

fn main() {
    loop {
        repl::start(std::io::stdin(), std::io::stdout());
    }
}
