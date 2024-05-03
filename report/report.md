# mini-x report

**Course Code: BSDSESM1KU**

| Student  | Email| 
| -------- | -------- | 
| Marius Thomsen          | mariu@itu.dk | 
| Marius W. S. Nielsen    | mawn@itu.dk  | 
| David A. Feldner        | dafe@itu.dk  | 
| Markus Grand Petersen   | mgrp@itu.dk  | 
| Michael Daniel Fabricius| midf@itu.dk  | 

## Systems perspective

### Description of the project
mini-x is a blazingly fast twitter/x clone written in Rust with the Actix framework. 

### Design and Architechture of the project
mini-x is designed with focus on performance, scalability and type safety. The application is structured into two main parts: the API server and the frontend client, each running concurrently on separate threads as seen in main.rs.
#### API Server
The API server is built using Actix-Web, a really fast web framework for Rust. A middleware for logging is wrapped around the app in the HTTP server setup, and PrometheusMetrics is configured for monitoring. 

#### Frontend Client


### Dependencies of mini-x

- Rust and Actix-web: 
    - The backend service is written in Rust, with Actix-web as the chosen framework due to its performance and ease of use in developing web apps. Actix-web handles the HTTP requests and routing
- PostgreSQL
    - Used for data storage in a robust and scalable way.
- Diesel.rs:
    - Diesel was used for ORM to ensure safe database interactions. Diesel provides type safety and convinient DSL for rust, such that complex SQL queries can be constructed safely.
- Docker and Docker Swarm:
    - Docker is used for containerization to ensure that the app runs identically across varying environments. Docker swarm manages a cluser of Docker Engines so we can spread workload horizontally.
- ELK stack: 
    - ElasticSearch, Logstash, Kibana and Beats was used for logging
- Prometheus and Grafana
    - Prometheus collects data from our api and frontend. The data is then shown in grafana

### Current state of mini-x


### Important interactions of sub systems
```
Make UML Sequence diagram that shows the flow of information through your system from user request in the browser, over all subsystems, hitting the database, and a response that is returned to the user.
``` 

```
Make illustrative sequence diagram that shows how requests from the simulator traverse your system.
```

## Process' persepctive

### CD/CI Explanation 
```
A complete description of stages and tools included in the CI/CD chains, including deployment and release of your systems.
```
### Monitering
```
How do you monitor your systems and what precisely do you monitor?
```
### Logging 
```
What do you log in your systems and how do you aggregate logs?
```
### Security 
```
Brief results of the security assessment and brief description of how did you harden the security of your system based on the analysis
```
## Security
### Risk Identification
#### Assets in our system. 
In our system, there are six virtual machines hosted on Digital Ocean. Five of them hold an interest in a malicious party. Monitoring provides all endpoints 
#### Assets and their value
- Application: The application has three replicas on three separate virtual machines. 
 - public information is found here, including usernames.
 - 3 nodes worth of computing power
- Database: A single virtual machine with a backup
 - All our data, hashed passwords, email, usernames, all messages
- Logging: A single VM with all our logs and errors.
- Users: The users on the application
 - Provides value.

#### Threats and Risks to Assets 
- Application: 
 - DDOS: our application can handle many requests per second depending on the endpoint. 
 - While our service can handle the simulator and then some. We could put all our VMs to full load with one machine running FFUF in Kali, targeting computationally heavy endpoints.
- Database:
 - Injection: All fields are sanitized. The ORM we use is injection-safe. The one SQL query we have uses prepared states.
 - Hashed passwords: Here we use bcrypt to encrypt them with salted hashing
 - Man in the middle: We send our data from the application to the database using HTTP
- Logging: 
 - Verbose error messages: having better responses from your other attempt will enable better attacks. 
 - GDPR theft: Some user data can be acquired.
- Uptime: 
 - Our system is vulnerable to DDos attack affection up time. Decreased will affect the number of users.
- Users: 
 - Obscene content: There is no content filter, all content is allowed, which could cause users to leave
 - no service: If our service is down, users leave 
 - no content: Without content, users don't stay
### IaC Strategy 
```
Applied strategy for scaling and upgrades
```

## Lessons learned perspective
```
Describe the biggest issues,
how you solved them, and which are major lessons learned with regards to:
Evolution and refactoring, Operation and Maintenence
of your ITU-MiniTwit systems. 
Link back to respective commit messages, issues, tickets, etc.
to illustrate these.

Also reflect and describe what was the "DevOps" style of your work.
For example, what did you do differently to previous development projects 
and how did it work?
```

### Evolution and refactoring

### Operation

### Maintenence

## Usage of LLM's in mini-x
```
Mention LLM tools how and where we used them and which ones did they help, speed up or slow us down. 
```