## Risk Identification
### Assets in our system. 
In our system, there are six virtual machines hosted on Digital Ocean. Five of them hold an interest in a malicious party. Monitoring provides all endpoints 
### Assets and their value
- Application: The application has three replicas on three separate virtual machines. 
 - public information is found here, including usernames.
 - 3 nodes worth of computing power
- Database: A single virtual machine with a backup
 - All our data, hashed passwords, email, usernames, all messages
- Logging: A single VM with all our logs and errors.
- Users: The users on the application
 - Provides value.

### Threats and Risks to Assets 
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
 

