extern mod std;
use from_str::FromStr;
use io::*;
use rand::*;

#[deriving_eq]
enum YesNo { Yes, No }

impl YesNo: FromStr {
  static pure fn from_str(s: &str) -> Option<YesNo> {
    match s.trim().to_lower() {
      ~"y" | ~"yes" => Some(Yes),
      ~"n" | ~"no" => Some(No),
      _ => None
    }
  }
}

#[deriving_eq]
enum Guess = int;
enum MyNumber = int;

#[deriving_eq]
enum GuessResponse { TooLow, TooHigh, JustRight }

impl Guess: FromStr {
  static pure fn from_str(s: &str) -> Option<Guess> {
    let i:Option<int> = FromStr::from_str(s.trim());
    i.map( |&g| Guess(g) )
  }
}

impl Guess {
  pure fn num(&self) -> int { **self }
}

impl MyNumber {
  pure fn num(&self) -> int { **self }

  pure fn respond(&self, guess: Guess) -> GuessResponse {
    let my_num = self.num();
    let guess_num = guess.num();

    if guess_num > my_num {
      TooHigh
    } else if guess_num < my_num {
      TooLow
    } else {
      JustRight
    }
  }
}

fn prompt<A: FromStr Copy>(message: &str, error_message: &str) -> A {
  loop {
    io::stdout().write_line(message);
    io::stdout().write_str("> ");
    io::stdout().flush();

    let line = io::stdin().read_line();
    let answer = FromStr::from_str(line);

    match answer {
      Some(value) => { return value }
      _ => { io::stdout().write_line(error_message) }
    }
  }
}

fn main() {
  loop {
    let new_game:YesNo = prompt("Would you like to play a game?",
                                "Please answer yes or no");

    match new_game {
      Yes => {
         println("Ok, let's play");
         play_game();
      }
      No => {
        println("Goodbye!");
        break;
      }
    }
  }
}

fn play_game() {
  let rng = rand::task_rng();
  let my_number = MyNumber(rng.gen_int_range(1,101));
  println("Ok, I've got my number. It's between 1 and 100.");

  loop {
    let guess:Guess = prompt("What is your guess?",
                             "Please enter a whole number");

    let response = my_number.respond(guess);

    match response {
      TooHigh => { println("Too High!") }
      TooLow => { println("Too Low!") }
      JustRight => {
        println("You got it!");
        break;
      }
    }
  }
}

#[test]
fn guess_from_str_works() {
  assert FromStr::from_str("1") == Some(Guess(1));
  assert FromStr::from_str("  1  ") == Some(Guess(1));
  let foo: Option<Guess> = FromStr::from_str("Foo");
  assert foo == None;
}

#[test]
fn yesno_from_str_works() {
  assert FromStr::from_str("y") == Some(Yes);
  assert FromStr::from_str("n") == Some(No);
  assert FromStr::from_str("Y") == Some(Yes);
  assert FromStr::from_str("yes") == Some(Yes);
  assert FromStr::from_str("  yes  ") == Some(Yes);
  assert FromStr::from_str("N") == Some(No);
  assert FromStr::from_str("no") == Some(No);
  let foo: Option<YesNo> = FromStr::from_str("yessir");
  assert foo == None;
}

#[test]
fn my_number_responds_too_high() {
  assert MyNumber(30).respond(Guess(31)) == TooHigh;
}

#[test]
fn my_number_responds_too_low() {
  assert MyNumber(30).respond(Guess(29)) == TooLow;
}

#[test]
fn my_number_responds_just_right() {
  assert MyNumber(30).respond(Guess(30)) == JustRight;
}
