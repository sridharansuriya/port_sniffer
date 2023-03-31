#### Port Sniffer written in Rust using clap
____________________________________________


This is a simple port sniffer application written in rust using CLAP crate and the standard tcp create.  

It is multi-threaded. i.e we can define the number of jobs to execute for port sniffing.

Eg. 
`port_sniffer --help`

<ins>Usage:</ins> port_sniffer.exe [OPTIONS] \<IP\>

<ins>Arguments:</ins>  
&nbsp;&nbsp;\<IP\>

<ins>Options:</ins>  
&nbsp;&nbsp;-j \<JOB\> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[default: 64]  
&nbsp;&nbsp;-h, --help&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Print help  
&nbsp;&nbsp;-V, --version&nbsp;&nbsp;Print version

