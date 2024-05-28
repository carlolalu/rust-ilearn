# rust-ilearn
A repo where I learn rust.

## Structure of the repo

The folder `WaStE` contains trash examples of draft code which is not worthy to be watched, but still testifies I poked it and played around with it. The other three folders contain exercises which might not be very clean, but are worthy to be saved not as 'waste'.

## Sources

### the oreilly book.
It totally has the advanced level required, but maybe properly because of this it is too advanced. Chapter 2 exposes me to concepts which I have no idea about, in particular about networking and concurrency (of which I would still like to learn one day). The book has no exercises but these examples, again.

- bookmark: 20 - an async client and server - The Server’s Main Function
- bookmark: 20 - Primitive Futures and Executors + Pinning - beginning
- left (temporarily?) behind: 10,   12,13,   16,17,  21,22,23

### the official asynchronous book
https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html

### the official async_std book (which mentions the previous book)
https://book.async.rs/introduction

### the official tokio tutorial
https://tokio.rs/tokio/tutorial

## plan
you MUST do projects to really learn

- ch 20 form oreilly,
- official async book with its exercises
- exercise with std::net. An idea is monitoring certain data and then plot some analysis on it. Use the other books you have.
- tokio official website - tutorial(s)
- design patterns
- ex with tokio async (kurose? docs on rust? echoserver? black hat rust and other books?). In this way u get to use both tokio and some plotting library.
- more on rayon (official website/crate?)
- more on networks (kurose? docs on rust? echoserver? black hat rust? and other books?). Suggestion, the book `Network programming with Rust` and `Rust for Network Programming`

In general my directions seems: 
1. tokio, concurrency and networks oriented programming (web scraping? penetration testing?)
2. physical concurrent modelling with visualization (rayon, plotting, csv files, data?)
3. draw fractals and work with images and gif (search a fractal and implement it)

## exercises
Start always easy with the KISS philosophy, and dive into the deeper and more technical stuff later. Try as much as you can, play, do projects.

### sources: 
http://nifty.stanford.edu/, 
exercisism,
ch2 oreilly,
codewars,
rapier tutorial
online walkthrough: https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb ,
walkthrough for simple project with some ai (chatgpt, hugging face)
framework to calculate averages for twice double degree students like me

### ideas: 
crate `image` at the section 'generating fractals' in the README.md
telegram-bot,
visualization things,
physical models, 
advised by victor: build programs to operate on a text-based web-browser, as https://en.wikipedia.org/wiki/W3m ,
plotting stuff with nalgebra and plotters
web scrapying asynchronously to gather data, and running parallel to analyse such data (use surf, reqwest, aync-std, rayon etc...)
well suited projects with duration, skills and code: https://www.placementpreparation.io/blog/rust-project-ideas-for-beginners/#files-compression-and-decompression
cool ideas for beginner projects with rust: https://zerotomastery.io/blog/rust-practice-projects/


### networking tutorials:
discord bot: 
https://github.com/serenity-rs/serenity/tree/current/examples
general networking: 
https://www.freecodecamp.org/news/computer-networking-how-applications-talk-over-the-internet/
https://daily.dev/blog/coding-project-ideas-for-networking
telegram bot: 
https://www.process.st/telegram-bot/
