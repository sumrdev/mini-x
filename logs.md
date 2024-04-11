# Refactoring session

## Week 1

### Upgrading minitwit.py to Python 3

In order to upgrade minitwit.py to Python 3, we ensure that the response data is handled correctly. Python 3 uses explicit distinction between bytes of binary data and str data. Python 3 also treats all strings as Unicode by default. Therefore we call rv.data.decode() in order to interpret the binary response data as text. For the same reasons in init_db() we have to .decode() the data that f.read() returns, since it is returned as bytes. Also there were a few syntax differences with brackets and such which we fixed in order to complete the Python 3 upgrade.

### Choosing a language

We chose rust, its blazingly fast!

The reasoning behind this, came from the multiple sources online claiming high performance, memory safety and code which is less likely to fail in production. Rust has a lot of documentation and community support, and many frameworks which aid in the many aspects of web app development.
### Choosing a framework

We looked at rocket and actix, however the actix framework seemed to have better performance and scalability as opposed to rocket. The decision was made based on the analysis made here https://www.techempower.com/benchmarks/#section=data-r21&hw=ph&test=fortune where actix-http is the 2nd most performant rust-compatible framework.

## Week 2

### Starting Rust refactor

The rust refactor starts with us getting familiar with rust in general, as it is a new language for all of us. 

We start by mapping the features of ITU-MiniTwit in featureOverview.txt as feature user stories such as "A user can register" etc. 

In order to get started codewise, we follow the doc.rust-lang.org "Getting Started" documentation, installing dependencies and setting up a template project including a main.rs file and the .html pages fetched from the original MiniTwit flask app. Then we adapt and extend the rust functions based on the MiniTwit flask app functions such that the functionality remains identical.

For the database side, we implement rustqlite which acts as a wrapper for using SQLite from Rust. 

### Dockerizing the rust refactor

The app is dockerized by adding a Dockerfile to expose port 5000 and binding the app to that port in main.rs. Thus all members of the team are able to develop in a uniform environment, so it is important to do this immediatly before development properly starts.

## Week 3

Implementation of the API begins, in order to prepare for the upcoming simulation. We found the specifications of the API in API_Spec/minitwit_sim_api.py which contains information on how the minitwit API works in the python environment, however we have to adapt this API specification to be functional with the Rust refactor we have made. We completed the transition of the API to Rust, utilizing libraries like actix-web for the server framework, chrono for time handling, and rusqlite for database interactions. We integrated features for session and cookie management using actix-session and maintained the API's functionality such as managing user registrations, message posting etc. appropriately. This ensures that the Rust API meets the same specifications and functionalities as the legacy Python API, now optimized for better performance and maintainability.

## Week 4

We finished up the work on the rust API in this week, and set up Docker compose, Vagrant, and Github Actions in order to achieve continous deployment with automatic pipelines for delivering value to customers immediately as a PR passes all check conditions (image builds, tests pass). In the choice between Continuous Integration (CI) and Continuous Deployment (CD), we therefore chose Continuous Deployment (CD). This approach allows us to automate the release of new features and updates directly to production as soon as the changes pass automated tests and build processes. By leveraging Continuous Deployment, we ensure that code changes are automatically and reliably released, reducing the cycle time for delivering updates and minimizing manual intervention. This aligns well with our goal of providing continuous value delivery to our customers without delay.

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
