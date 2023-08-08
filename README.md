# Aviora

Aviora is a CAD/MDT backend. In the future there will be some frontends such as a site but for now this is simply a backend for these apps. 

### **Current features**:

- `Registering a civ`: Each civ is given a unique state ID (For now, the only generation option is the US SSN). They also have what is called a civ-ref which is a sha256 hash of their snowflake ID and their first + last name. This can be used as an easier way to look up a players information for police (or other first responders/people who would need to find a player) in cad / mdt systems. The first 4 characters (This may be up'd in the future), the entire civ-ref, or their snowflake ID may be used to look up. Additionally, their name, but that's a given.