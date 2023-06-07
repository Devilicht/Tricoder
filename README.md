# Tricoder

This is a port scanner project developed in Rust. It allows enumerating subdomains of a given target and checking which ports are open on each subdomain.

## Project Structure

The project consists of several files, each with a specific function. Here's a description of each file:

- **main.rs**: This is the main file of the project. It is responsible for receiving the target as a command-line argument, setting up the HTTP client, and executing the subdomain enumeration and open port checking.

- **error.rs**: This file contains the definition of the Error type and its implementations. It handles errors related to incorrect program usage and HTTP requests.

- **model.rs**: In this file, the data models used by the program are defined. It contains the Subdomain structure, which represents a subdomain and its open ports, and the Port structure, which represents a port and its state (open or closed).

- **ports.rs**: Here, the port checking logic is implemented. The file contains the scan_ports function, which takes a subdomain and checks which ports are open. The scan_ports function uses the scan_port function to individually check each port.

- **subdomain.rs**: This file handles subdomain enumeration. It contains the enumerate function, which takes an HTTP client and a target, and returns a list of found subdomains. The enumerate function uses the resolves function to check if a subdomain is resolvable.

- **common_ports.rs**: In this file, the most common ports to be checked are defined. The MOST_COMMON_PORTS_100 constant contains a list of port numbers.

## Usage

To run the program, you need to pass the target as a command-line argument. The target should be provided in the form of a domain name. For example:
```sh
tricoder example.com
```

The program will perform subdomain enumeration and check open ports for each found subdomain. The results will be displayed in the console.
Multithreading

This project utilizes multithreading to improve performance. It uses the rayon library, which provides a convenient way to parallelize operations across multiple threads. The program utilizes a custom thread pool with 256 threads to distribute the workload efficiently.

By leveraging multithreading, the port scanning process can be significantly accelerated, allowing for faster enumeration and checking of open ports on multiple subdomains simultaneously.
Dependencies

The project depends on the following external libraries:

-  **thiserror**: A library to simplify the definition of custom errors.
-  **serde**: A library for serialization and deserialization of data.
-  **reqwest**: A library for making HTTP requests.
-  **rayon**: A library for parallelizing operations across multiple threads.
-  **trust-dns-resolver**: A library for DNS name resolution.

Make sure to have the dependencies installed before compiling and running the project.

