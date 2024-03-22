# Refactoring session

## Week 1

### Choosing a language

We chose rust, its blazingly fast!

### Choosing a framework

We looked at rocket and actix, actix looked better to us so we chose it.

### Actually refactoring

First we needed to figure out how anything worked. None of us ever used rust before.
We figured out how to get a Hello world up on "/"!
Then we struggled a bit to get templates up and working, we figured out we needed to have structs for all the data on the site.
Flashes got up and working after a bit of fiddling
After we got templates up and running we needed to get sessions working. We looked at a few methods of getting it to work.
Found nice library for it!
Then we got the database working, this was not too bad. Excetp we had to figure out how to get the real fields out instead of random ones.

## Week 2
### API

For the API we just created a seperated actix project that worked independantly from the web app. 
Having knowledge and code from developing the web app made implementing the API very smooth.
The implementation of default values for the query parameters "no" and "latest" didn't work.
We skipped denying unauthorized requests as the test suite didn't cover it.

## Week 3

## Week 4

## Week 5

## Week 6

## Week 7

## Week 8

## Week 9 

## Week 10

## Week 11
