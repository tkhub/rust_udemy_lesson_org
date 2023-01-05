mod filters
{
    pub struct PolarCoordinate
    {
        Real:f64,
        Imaginary:f64,
    }

    pub mod lpf_1st
    {
        pub fn frequency_characteristic(cutoff:f64) -> f64
        {
            println!("{}", cutoff);
            0.0
        }
    }
}
pub fn test()
{
    println!("TEST");
}