//
// zhtta.rs
//
// Running on Rust 0.8
//
// Starting code for PS3
// 
// Note: it would be very unwise to run this server on a machine that is
// on the Internet and contains any sensitive files!
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use std::rt::io::*;
use std::rt::io::net::ip::SocketAddr;
use std::io::println;
use std::cell::Cell;
use std::{os, str, io};
use extra::{arc, priority_queue};
use std::comm::*;

static PORT:    int = 4414;
static IP: &'static str = "0.0.0.0"; 

struct sched_msg {
    in_charlottesville: bool,
    file_req_size: uint,
    request_id: uint,
    stream: Option<std::rt::io::net::tcp::TcpStream>,
    filepath: ~std::path::PosixPath
}

impl Ord for sched_msg {
    // Higher priority programs are given priority on the queue.
    fn lt(&self, other: &sched_msg) -> bool { 
	if (self.in_charlottesville ^ other.in_charlottesville) {
	    !self.in_charlottesville && other.in_charlottesville
		// If you are not in charlottesville but other is, let other go first
	}
	else if (self.file_req_size != other.file_req_size) {
	    self.file_req_size > other.file_req_size 
		// If your file size is bigger than other's, let other go first.
	}
	else {
	    self.request_id > other.request_id
		//tie breaker is id, oldest request goes first.
	}
    }
}

fn main() {
    println("Starting server...");
    let req_vec: ~[sched_msg] = ~[];
    let req_vec = priority_queue::PriorityQueue::from_vec(req_vec);
    let shared_req_vec = arc::RWArc::new(req_vec);
    let add_vec = shared_req_vec.clone();
    let take_vec = shared_req_vec.clone();

    let (port, chan) = stream();
    let chan = SharedChan::new(chan);

    // add file requests into queue.
    do spawn {
	loop {
	    do add_vec.write |vec| {
		    let tf:sched_msg = port.recv();
		    (*vec).push(tf);
		    println(fmt!("add to queue with queue size = %u", (*vec).len()));
	    }
	}
    }

    // take file requests from queue, and send a response.
    do spawn {
	loop {
	    do take_vec.write |vec| {
		if (*vec).len() > 0 {
		    let mut tf = (*vec).pop();
		    println(fmt!("popped from queue with queue size = %u", (*vec).len()));

		    match io::read_whole_file(tf.filepath) {
			Ok(file_data) => {
			    println(fmt!("begin serving file [%?]", tf.filepath));
			    tf.stream.write(file_data);
			    tf.stream.write("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream; charset=UTF-8\r\n\r\n".as_bytes());
			    tf.stream.write(file_data);
			    println(fmt!("finish file [%?]", tf.filepath));
			}
			Err(err) => {
			    println(err);
			}
		    }
		}
	    }
	}
    }

    let ip = match FromStr::from_str(IP) { 
	Some(ip) => ip, 
	None => { println(fmt!("Error: Invalid IP address <%s>", IP)); return;},
    };
    let socket = net::tcp::TcpListener::bind(SocketAddr {ip: ip, port: PORT as u16});
    //let socket = net::tcp::TcpListener::bind(SocketAddr {ip: Ipv4Addr(127,0,0,1), port: PORT as u16});

    println(fmt!("Listening on tcp port %d ...", PORT));
    let mut acceptor = socket.listen().unwrap();

    let visitor_count_master = arc::RWArc::new(0);

    // we can limit the incoming connection count.
    //for stream in acceptor.incoming().take(10 as uint) {
    for stream in acceptor.incoming() {
	let stream = Cell::new(stream);

	// Start a new task to handle the connection
	let child_chan = chan.clone();
	let visitor_count = visitor_count_master.clone();
	do spawn {
	    visitor_count.write( |count| { *count += 1;} );

	    let mut stream = stream.take().unwrap();

	    let in_charlottesville = match stream.peer_name() {
		Some(pn) => {
		    let s = pn.to_str();
		    s.slice_chars(0, 6) == "137.54" || s.slice_chars(0,7)=="128.143"
		},
		None => false
	    };

	    let mut buf = [0, ..500];
	    stream.read(buf);
	    let request_str = str::from_utf8(buf);

	    let req_group : ~[&str]= request_str.splitn_iter(' ', 3).collect();
	    if req_group.len() > 2 {
		let path = req_group[1];
		println(fmt!("Request for path: \n%?", path));
		let file_path = ~os::getcwd().push(path.replace("/../", ""));
		if !os::path_exists(file_path) || os::path_is_dir(file_path) {
		    println(fmt!("Request received:\n%s", request_str));
		    let response: ~str = fmt!(
			"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
			 <doctype !html><html><head><title>Hello, Rust!</title>
			 <style>body { background-color: #111; color: #FFEEAA }
				h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
				h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
			 </style></head>
			 <body>
			 <h1>Greetings, Krusty!</h1>
			 <h2>Visitor count: %u</h2>
			 </body></html>\r\n", visitor_count.read(|count| { *count }) );

		    stream.write(response.as_bytes());
		}
		else {
		    // may do scheduling here
		    let file_size = match io::file_reader(file_path) {
			Ok(file) => {
			    file.seek(0, io::SeekEnd);
			    file.tell()
			}
			Err(err) => {
			    println(err);
			    0u
			}
		    };
		    let req_id = visitor_count.read(|count| { *count });
		    let msg: sched_msg = sched_msg{
			file_req_size: file_size, 
			in_charlottesville: in_charlottesville, 
			request_id: req_id,
			stream: Some(stream), 
			filepath: file_path.clone()
		    };
		    child_chan.send(msg);

		    println(fmt!("get file request: %?", file_path));
		}
	    }
	    println!("connection terminates")
	}
    }
}
