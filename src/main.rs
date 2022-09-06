use std::fs::File;
use std::io::prelude::*;
use std::process;

use sargparse::{ArgumentParser, ArgumentType, InnerData};
use ureq;

mod libEnum;

fn main() {
    let mut parser = ArgumentParser::new(Some("A simple Dir and Subdomain Enumerator !"));

    parser.add_argument("-d", "--directory", "Use this option to set the Dir enumerator and pass the website like a domain, like this: http://test.com, https://bancocn.com", false, Some(InnerData::STR(String::from("null"))), ArgumentType::STR);
    parser.add_argument("-D", "--subDomain", "Use this option to set the subdomain enumerator and pass the website like a domain, like this: test.com, bancocn.com", false, Some(InnerData::STR(String::from("null"))), ArgumentType::STR);
    parser.add_argument("-w", "--wordlist", "Pass the wordlist file, is obrigatory !!", false, Some(InnerData::STR(String::from("null"))), ArgumentType::STR);
    parser.add_argument("-a", "--userAgent", "Pass the user-agent", false, Some(InnerData::STR(String::from("null"))), ArgumentType::STR);
    parser.add_argument("-p", "--proxy", "Pass the proxy socks5, like this: socks5://user:password@cool.proxy:9090", false, Some(InnerData::STR(String::from("null"))), ArgumentType::STR);    
    parser.add_argument("-o", "--output", "Set the name of output file !", false, Some(InnerData::STR(String::from("null"))), ArgumentType::STR);

    let args = parser.parse_args().unwrap();

    if args.get("wordlist").unwrap().get_str() == "null" {
        println!("Pass the wordlist file, is obrigatory to do Enumerations !!");
        process::exit(1);

    }else {

        let wordlist: String = args.get("wordlist").unwrap().get_str();

        if args.get("directory").unwrap().get_str() != "null"{
            let url: String = args.get("directory").unwrap().get_str();

            let userAgent: String;
            let proxy: String;
            let outputFile: String;

            
            if args.get("userAgent").unwrap().get_str() != "null"{
                userAgent = args.get("userAgent").unwrap().get_str();
               

            }else {
                userAgent = String::from("null");
               
            }


            if args.get("proxy").unwrap().get_str() != "null"{
                proxy = args.get("proxy").unwrap().get_str();
              

            }else {
                proxy = String::from("null");
             

            }


            if args.get("output").unwrap().get_str() != "null"{
                outputFile = args.get("output").unwrap().get_str();
             

            }else {
                outputFile = String::from("null");
                

            }

            println!("=====================================================");
            println!("Hunt - Subdomain and Dir Hunter       By ShadowHunter");
            println!("=====================================================");
            println!("[+] Enum Type: Dir");
            println!("[+] Url / Domain: {}", url);
            println!("[+] Wordlist: {}", wordlist);

            if userAgent == "null"{
                println!("[+] User-Agent: No user-agent");

            }else {
                println!("[+] User-Agent: {}", userAgent);

            }

            if proxy == "null" {
                println!("[+] Proxy Socks5: No proxy");

            }else {
                println!("[+] Proxy Socks5: {}", proxy);

            }
            println!("==================|Doing The Enum|===================\n");

         
            libEnum::Enums::enumDir(url, wordlist, userAgent, proxy, outputFile);
            

        }else if args.get("subDomain").unwrap().get_str() != "null" {
            let url: String = args.get("subDomain").unwrap().get_str();

            let proxy: String;
            let outputFile: String;

            if args.get("userAgent").unwrap().get_str() != "null"{
                println!("In one subdomain enum don't need a userAgent...");
                process::exit(1);

            }

            if args.get("proxy").unwrap().get_str() != "null"{
                println!("In a subdomain enum don't need a proxy, we recommended use the DNS on Tor Network, in this form the privacy is preserved, the enumeration use the DNS configuration of file => /etc/resolv.conf");
                process::exit(1);
              

            }


            if args.get("output").unwrap().get_str() != "null"{
                outputFile = args.get("output").unwrap().get_str();
             

            }else {
                outputFile = String::from("null");
                
                
            }

            println!("=====================================================");
            println!("Hunt - Subdomain and Dir Hunter       By ShadowHunter");
            println!("=====================================================");
            println!("[+] Enum Type: subDomain");
            println!("[+] Url / Domain: {}", url);
            println!("[+] Wordlist: {}", wordlist);
            println!("==================|Doing The Enum|===================\n");

            libEnum::Enums::enumSub(url, wordlist, outputFile);


        }else if args.get("directory").unwrap().get_str() != "null" && args.get("subDomain").unwrap().get_str() != "null"{
            println!("only one enumeration at a time...");
            process::exit(1);

        }else {
            println!("Pls pass one option -d or -D to do one enumeration...");
            process::exit(1);

        }

    }
}
