use std::process;
use std::fs::File;
use std::io::prelude::*;

use ureq;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;


pub struct Enums;

impl Enums {
    pub fn enumDir(url: String, wordlist: String, userAgent: String, proxy: String, outputFile: String){
        let mut file = File::open(wordlist).expect("could not found the wordlist file !");
        let mut content = String::new();
        
        file.read_to_string(&mut content);

        let wordlist = content.split("\n");

        let response = ureq::get(&url).set("User-Agent", "Hunt WebEnum Tool").call();

        match response {
            Ok(_) => {
                if userAgent != "null" && proxy != "null" && outputFile != "null"{
                    let mut fileOutput = File::create(outputFile).expect("Could't create the output file...");
                    let proxySocks = ureq::Proxy::new(format!("socks5://{}", proxy));

                    match proxySocks {
                        Ok(proxySocks) => {
                            let agent = ureq::AgentBuilder::new()
                                .proxy(proxySocks)
                                .build();

                            for word in wordlist {
                                let response = agent.get(&format!("{}/{}", url, word)).set("User-Agent", &userAgent).call();

                                match response {
                                    Ok(resp) => {
                                        if resp.status() == 200 {
                                            println!("{}/{}\r", url, word);
                                            fileOutput.write_all(format!("{}/{}\r", url, word).as_bytes()).expect("Can't write in file !!");
                
                                        }else if resp.status() != 404 {
                                            println!("{}/{} ({})\r", url, word, resp.status());
                                            fileOutput.write_all(format!("{}/{} ({})\r", url, word, resp.status()).as_bytes()).expect("Can't write in file !!");
        
                                        }
                                    },

                                    Err(_) => {
                                        continue;

                                    }
                                }
                            }
                        }, 

                        Err(err) => {
                            println!("{}", err);
                            println!("can't possible use this socks");

                        }
                    }


                }else if userAgent != "null" && outputFile != "null"{
                    let mut fileOutput = File::create(outputFile).expect("Could't create the output file...");

                    for word in wordlist {
                        let response = ureq::get(&format!("{}/{}", url, word)).set("User-Agent", &userAgent).call();

                        match response{
                            Ok(resp) => {
                                if resp.status() == 200 {
                                    println!("{}/{}\r", url, word);
                                    fileOutput.write_all(format!("{}/{}\r", url, word).as_bytes()).expect("Can't write in file !!");
        
                                }else if resp.status() != 404 {
                                    println!("{}/{} ({})\r", url, word, resp.status());
                                    fileOutput.write_all(format!("{}/{} ({})\r", url, word, resp.status()).as_bytes()).expect("Can't write in file !!");

                                }
                            },

                            Err(_) => {
                                continue;

                            }
                        }
                    }

        
                }else if proxy != "null" && outputFile != "null" { 
                    let mut fileOutput = File::create(outputFile).expect("Could't create the output file...");
                    let proxySocks = ureq::Proxy::new(&format!("socks5://{}", proxy));

                    match proxySocks {
                        Ok(proxySocks) => {
                            let agent = ureq::AgentBuilder::new()
                                .proxy(proxySocks)
                                .build();

                            for word in wordlist {
                                let response = agent.get(&format!("{}/{}", url, word)).set("User-Agent", "Hunt WebEnum Tool").call();

                                match response {
                                    Ok(resp) => {
                                        if resp.status() == 200 {
                                            println!("{}/{}\r", url, word);
                                            fileOutput.write_all(format!("{}/{}\r", url, word).as_bytes()).expect("Can't write in file !!");
                
                                        }else if resp.status() != 404 {
                                            println!("{}/{} ({})\r", url, word, resp.status());
                                            fileOutput.write_all(format!("{}/{} ({})\r", url, word, resp.status()).as_bytes()).expect("Can't write in file !!");

                                        }

                                    },

                                    Err(_) => {
                                        continue;

                                    },
                                }
                            }
                        },

                        Err(err) => {
                            println!("{}", err);
                            println!("can't possible use this socks");

                        }
                    }

                    
                }else if userAgent != "null" && proxy != "null"{
                    let proxySocks = ureq::Proxy::new(&format!("socks5://{}", proxy));

                    match proxySocks {
                        Ok(proxySocks) => {

                            let agent = ureq::AgentBuilder::new()
                                .proxy(proxySocks)
                                .build();

                            for word in wordlist {
                                let response = agent.get(&format!("{}/{}", url, word)).set("User-Agent", &userAgent).call();

                                match response {
                                    Ok(resp) => {
                                        if resp.status() == 200 {
                                            println!("{}/{}\r", url, word);
                
                                        }else if resp.status() != 404 {
                                            println!("{}/{} ({})\r", url, word, resp.status());
                
                                        }
                                    },

                                    Err(_) => {
                                        continue;

                                    },
                                }
                            }
                        },

                        Err(err) => {
                            println!("{}", err);
                            println!("can't possible use this socks");
                            
                        },
                    }

        
                }else if userAgent != "null"{
                    for word in wordlist {
                        let response = ureq::get(&format!("{}/{}", url, word)).set("User-Agent", &userAgent).call();
                        
                        match response {
                            Ok(resp) => {
                                if resp.status() == 200 {
                                    println!("{}/{}\r", url, word);
        
                                }else if resp.status() != 404 {
                                    println!("{}/{} ({})\r", url, word, resp.status());
        
                                }
                            },

                            Err(_) => {
                                continue;

                            },
                        }
                    }
                    
                
                }else if proxy != "null"{
                    let proxySocks = ureq::Proxy::new(&format!("socks5://{}", proxy));

                    match proxySocks {
                        Ok(proxySocks) => {
                            let agent = ureq::AgentBuilder::new()
                                .proxy(proxySocks)
                                .build();
                    

                            for word in wordlist{
                                let response = agent.get(&format!("{}/{}", url, word)).set("User-Agent", "Hunt WebEnum Tool").call();

                                match response {
                                    Ok(resp) => {
                                        if resp.status() == 200 {
                                            if resp.status() == 200 {
                                                println!("{}/{}\r", url, word);
                    
                                            }else if resp.status() != 404 {
                                                println!("{}/{} ({})\r", url, word, resp.status());
                    
                                            }
                                        }

                                    },
    
                                    Err(_) => {
                                        continue;
    
                                    },
                                }
                            }   
                        },

                        Err(err) => {
                            println!("{}", err);
                            println!("can't possible use this socks");
                            
                        },
                    }


                }else if outputFile != "null"{
                    let mut fileOutput = File::create(outputFile).expect("Could't create the output file...");

                    for word in wordlist {
                        let response = ureq::get(&format!("{}/{}", url, word)).set("User-Agent", "Hunt WebEnum Tool").call();

                        match response {
                            Ok(resp) => {
                                if resp.status() == 200 {
                                    println!("{}/{}\r", url, word);
                                    fileOutput.write_all(format!("{}/{}\n", url, word).as_bytes()).expect("Can't write in file");
        
                                }else if resp.status() != 404 {
                                    println!("{}/{} ({})\r", url, word, resp.status());
                                    fileOutput.write_all(format!("{}/{} ({})\n", url, word, resp.status()).as_bytes()).expect("Can't write in file");

                                }
                            },

                            Err(err) => {
                                continue;

                            },
                        }
                    }


                }else {
                    for word in wordlist{
                        let response = ureq::get(&format!("{}/{}", url, word)).set("User-Agent", "Hunt WebEnum Tool").call();

                        match response{
                            Ok(resp) => {
                                if resp.status() == 200 {
                                    println!("{}/{}\r", url, word);
        
                                }else if resp.status() != 404 {
                                    println!("{}/{} ({})\r", url, word, resp.status());
        
                                }
                            },
        
                            Err(_) => {
                                continue;
        
                            },
                        }
                    }
                }
            },

            Err(err) => {
                println!("{}", err);
                process::exit(1);

            },
        } 
    }

    pub fn enumSub(url: String, wordlist: String, outputFile: String) {
        let mut file = File::open(wordlist).expect("could not found the wordlist file !");
        let mut content = String::new();
        
        file.read_to_string(&mut content);

        let wordlist = content.split("\n");

        let resolver = Resolver::from_system_conf();

        match resolver {
            Ok(resolver) => {
                if outputFile != "null" {
                    let mut file = File::create(outputFile).expect("Could't create the output file...");

                    for word in wordlist {
                        let response = resolver.lookup_ip(&format!("{}.{}", word, url));

                        match response {
                            Ok(resp) => {
                                println!("{}.{} ====> {}", word, url, resp.iter().next().expect("an unexpected error occurred"));
                                file.write_all(format!("{}.{} ====> {}\n", word, url, resp.iter().next().expect("an unexpected error occurred").to_string()).as_bytes()).expect("Can't write in file !");

                            }, 
        
                            Err(_) => {
                                continue;
        
                            },
                        }
                    }
                    

                }else {
                    for word in wordlist {
                        let response = resolver.lookup_ip(&format!("{}.{}", word, url));
                                
                        match response {
                            Ok(resp) => {
                                println!("{}.{} ====> {}", word, url, resp.iter().next().expect("an unexpected error occurred"));
        
                            }, 
        
                            Err(_) => {
                                continue;
        
                            }
                        } 
                    }     
                }
            },

            Err(err) => {
                println!("{}", err);
                    println!("Can't create the resolver !!");
                    process::exit(1);

            }
        }
    }
}