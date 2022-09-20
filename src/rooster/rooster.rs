
struct Rooster {
    connData: RoosterConn,
    connectionPool: Pool,
    config: RoosterConfig,
}

impl RoosterTrait {
    fn new(connData: RoosterConn, roosterConfig: Option<RoosterConfig>) -> Rooster {
        return Rooster {
            connData,
            config: roosterConfig.unwrap_or(RoosterConfig::default())
        }
    }
    fn connect(&self) {
        let url = &self.connData.toUrl()[..];
        let result = Pool::new(url);
        self.connectionPool = match result {
            Ok(pool) => pool, 
            Err(err) => panic!("ROOSTER : connection error occured\n {}", err),
        };
    }
}

struct RoosterQueryFragment {
    last: Box<RoosterQueryFragment>,
}


