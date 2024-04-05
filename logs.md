# Refactoring session

## Week 1

### Upgrading minitwit.py to Python 3

In order to upgrade minitwit.py to Python 3, we had to ensure that the response data was handled correctly. Python 3 uses explicit distinction between bytes of binary data and str data. Python 3 also treats all strings as Unicode by default. Therefore we call rv.data.decode() in order to interpret the binary response data as text. For the same reasons in init_db() we have to .decode() the data that f.read() returns, since it is returned as bytes. Also there were a few syntax differences with brackets and such which we fixed in order to complete the Python 3 upgrade.

### Choosing a language

We chose rust, its blazingly fast!

The reasoning behind this, came from the multiple sources online claiming high performance, memory safety and code which is less likely to fail in production. Rust has a lot of documentation and community support, and many frameworks which aid in the many aspects of web app development.

### Choosing a framework

We looked at rocket and actix, however the actix framework seemed to have better performance and scalability as opposed to rocket. The decision was made based on the analysis made here https://www.techempower.com/benchmarks/#section=data-r21&hw=ph&test=fortune where actix-http is the 2nd most performant rust-compatible framework.

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

We choose diesel.rs for our ORM. Known for having your tables inside your code with structs meaning type safety, and other nice things like the table! macro. Can map rust code directly to sql queries, so you control your queries and not the ORM. Also has very useful documentation for getting started https://github.com/diesel-rs/diesel/tree/2.1.x/examples https://diesel.rs/guides/getting-started.
Using the gettings started examples we quickly got something basic up and running. It would however take some time learning to build queries in a new way. And refactoring the system to use these queries. We refactored the application such that the queries lie outside the main logic, and we just call the functions from there instead.

## Week 6

Using our ORM to change from sqlite to postgresql was not as simple as we would have liked.

- First we had to create a new server that ran postgres. We created a new droplet with DigitalOcean using vagrant.
- Then we fixed our dockerfiles to support this change. These two steps were pretty easy
- The actual queries we had written did not need any attention as the ORM did this for us.
- We however did have to port the DDL rust code to something new. As many sqlite fields were not compatible with postgres.
- We had to write a large SQL script that could port our tables over manually. Then we used a tool called pg-loader to automatically put the data inside our new postgres server.
- When deploying the service we used the default postgres port with a user named 'postgres' with a password of 'postgres' this silly error got our database constantly deleted by adversaries until we changed the password. Very unfortunate and silly mistake on our part.

## Week 7

## Week 8

## Week 9

## Week 10

## Week 11
