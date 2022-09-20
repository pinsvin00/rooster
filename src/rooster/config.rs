pub struct RoosterConfig {
    experimental: bool,
}

pub trait Config {
    fn default() -> Self;
}


impl Config for RoosterConfig {
    
    fn default() -> RoosterConfig {
        return RoosterConfig {
            experimental: true,
        }
    }
}