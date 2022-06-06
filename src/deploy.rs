pub mod deploy{

    use tilemap::TileConfig;

    pub struct Deploy{
        pub deploy_biome:HashMap< u16, BiomeConfig >,
        pub deploy_tile:HashMap< u16, TileConfig >,
    }




    pub fn new() -> Deploy{
        return Deploy{

        }
    }
}