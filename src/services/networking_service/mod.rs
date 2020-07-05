pub struct NetworkingService {

}

impl NetworkingService {
    pub fn new() -> NetworkingService {
        NetworkingService {

        }
    }

    pub fn update_servers(&mut self) {
        // let mut stream = TcpStream::connect("localhost:25565").unwrap();println!("{}", 0b00000011);
        //
        // let mut input: [u32; 128] = [0; 128];
        //
        // // Set version code to -1
        // input[0] = 0b00000011;
        //
        // let mut output: Vec<u8> = Vec::new();
        //
        // stream.write(&[0b00000011]).unwrap();
        // stream.read_to_end(&mut output).unwrap();
        //
        // for ret in output {
        //     println!("{:b} ", ret);
        // }
    }

    pub fn le_to_be(input: &mut [u8]) {
        for (i, item) in input.iter_mut().enumerate() {
            *item = u8::from_be(*item);
        }
    }

}