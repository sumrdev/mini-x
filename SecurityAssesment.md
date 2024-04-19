## Risk Identification
### Assets in our system. 
In out system we six virtual machines hosted on digital ocean 5 of them are interesting to a malicious party. Our monitering is pratical for us but it does not hold much vallue.
### Assets and their value
- Application: The applicaiton is hosted on three seperate virtual machines each hosted. 
  - Most information from can be found here
  - 3 nodes worth of Compute power
- Database: A single virtual machine with back up
  - All our data, hashed paswords, email, usernames, all messeges
- Logging: A single VM with all our logs and error.
  - Verbose error messeges
- uptime: 
  - value for users
- Users: The users on the application
  - Provides value 

### Threats and risks to Assets 
- Application: 
  - DDOS: our application can handle a quite a few requests per second deepending on the endpoint. 
    - While our service can handle the simulator and then some. couple hundred users. We could put all our VMs to full load with one machine running FFUF in kali targeting computaionally heavy endpoints
- Database:
  - Injection: All  our fiels are sanitized and the ORM we use is injecction safe, the one SQL query we have uses prepared states
  - Hashed paswords: here we use bcrypt to encrypt them with salted hashing
  - Man in the middle: We send our data from the application to the database ussing HTTP
- Logging: 
  - Verbose error messeges, having better responses form your other attempt will enable better attacks. 
  -  GDPR theft: Some user data could be found here
- Uptime: 
  - Our seystem is vulnurable to DDos attack affection up time which will affecct numper of users
- Users: 
  - Obceen content: there is no content filter and all content is allowed which could course users to leave
  - no service: If our service is down they leave 
  - no content: Without content user dont stay
  
## Risk Analysis
(https)
(man in the middle for db)

    Determine likelihood
    Determine impact
    Use a Risk Matrix to prioritize risk of scenarios
    Discuss what are you going to do about each of the scenarios
