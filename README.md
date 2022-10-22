# scaling-palm-tree
A command-line interface (CLI) for bus timetables in Wellington, New Zealand.

## Install Rust 

Rust can be installed on a Linux platform using the following command. 

```bash 
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Example 

When we run the client, we give the stop id and service number as arguments. 

```bash
# cargo run <stop_id> <service_number>
$ cargo run 5510 1 
```

Here is an example output given from the CLI. 

```
1    Island Bay      9min
1    Island Bay      9min    ♿
1    Island Bay      CAN
1    Island Bay      08:17AM
1    Island Bay      08:32AM
1    Island Bay      08:47AM
1    Island Bay      09:04AM
1    Island Bay      09:17AM
1    Island Bay      09:32AM
1    Island Bay      09:49AM
1    Island Bay      10:04AM
1    Island Bay      10:19AM
1    Island Bay      10:34AM
````

It is designed to mimic the Metservice Wellington Departure Timetables for a given stop. 

![departutre_board_IRL](https://user-images.githubusercontent.com/18411037/197334859-fb8c2db0-f3b2-4d22-845a-0e310d66112b.jpg)

