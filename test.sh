#!/bin/sh

rustc --test guesser.rs -o tests && ./tests
